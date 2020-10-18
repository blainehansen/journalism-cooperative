use actix_web::{post, http, web::{self, Json}};

use serde::{Serialize, Deserialize, Deserializer};
use validator::{Validate};

use sqlx::prelude::*;
type Pool = web::Data<sqlx::PgPool>;

fn base64_encode(s: &[u8]) -> String {
	base64::encode_config(s, base64::URL_SAFE)
}

fn get_env(env_var: &'static str) -> std::io::Result<String> {
	use std::io::{Error, ErrorKind};
	std::env::var(env_var)
		.map_err(|_| Error::new(ErrorKind::Other, format!("{} isn't set", env_var)))
		.map(|v| v.trim().to_string())
}

fn generate_random_token() -> std::io::Result<String> {
	use rand::prelude::*;
	let mut rng = rand::thread_rng();
	let mut buf: [u8; 64] = [0; 64];
	rng.try_fill(&mut buf)?;
	Ok(base64_encode(&buf[..]))
}

fn from_base64<'d, D>(deserializer: D) -> Result<String, D::Error>
	where D: Deserializer<'d>
{
	use serde::de::Error;
	let de = String::deserialize(deserializer)?;
	let buf = base64::decode_config(&de, base64::URL_SAFE).map_err(|_| Error::custom("couldn't decode from base64"))?;
	String::from_utf8(buf).map_err(|_| Error::custom("couldn't encode into utf8"))
}



lazy_static::lazy_static! {
	static ref MAILGUN_API_KEY: http::HeaderValue = {
		let contents = get_env("MAILGUN_API_KEY").expect("MAILGUN_API_KEY isn't set");
		let auth = base64_encode(contents.trim().as_bytes());
		http::HeaderValue::from_str(&format!("Basic {}", auth)).expect("couldn't construct valid header")
	};
}


#[derive(Debug)]
struct TerseInternalError;
impl std::fmt::Display for TerseInternalError {
	fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result { Ok(()) }
}
impl actix_web::error::ResponseError for TerseInternalError {
	fn error_response(&self) -> actix_web::HttpResponse {
		actix_web::dev::HttpResponseBuilder::new(self.status_code()).finish()
	}
	fn status_code(&self) -> http::StatusCode {
		http::StatusCode::INTERNAL_SERVER_ERROR
	}
}

// enum DatabaseError {
// 	Variant1,
// 	Variant2,
// }

// impl From<tokio_postgres::Error> for GenericError {
// 	fn from(error: tokio_postgres::Error) -> Self {
// 		let c = error.code();
// 		if c == Some(&tokio_postgres::error::SqlState::INTEGRITY_CONSTRAINT_VIOLATION) {
// 			GenericError::BadRequest
// 		}
// 		else if c == Some(&tokio_postgres::error::SqlState::UNIQUE_VIOLATION) {
// 			GenericError::NoContent
// 		}
// 		else {
// 			GenericError::InternalServer
// 		}
// 	}
// }


fn internal_error<E: std::fmt::Display>(e: E) -> TerseInternalError {
	log::error!("{}", e);
	TerseInternalError
}

fn invalid_rows(rows: u64) -> TerseInternalError {
	log::error!("database gave invalid rows_affected: {}", rows);
	TerseInternalError
}


#[derive(Debug, Validate, Deserialize)]
struct NewEmailJsonInput {
	#[validate(email)]
	email: String,
}
#[post("/subscribe")]
async fn subscribe(
	data: Json<NewEmailJsonInput>,
	pool: Pool,
) -> actix_web::Result<actix_web::HttpResponse> {
	match data.validate() {
		Ok(_) => (),
		Err(_) => {
			return Ok(actix_web::HttpResponse::BadRequest().finish());
		},
	};
	let verification_token = generate_random_token()?;
	let email = data.into_inner().email;

	let rows = sqlx::query!(
		r#"
			insert into subscription (email, verification_token) values ($1::text, $2)
			on conflict (email) do nothing
		"#,
		email, verification_token,
	)
		.execute(&**pool).await
		.map_err(internal_error)?
		.rows_affected();

	match rows {
		0 => { return Ok(actix_web::HttpResponse::NoContent().finish()); }
		1 => (),
		_ => Err(invalid_rows(rows))?,
	}

	// TODO validate success
	// {
	//   "id": "<20201014204400.1.EB5841F1F422C9CC@crowdsell.io>",
	//   "message": "Queued. Thank you."
	// }

	#[derive(Debug, Serialize)]
	struct MailgunForm {
		to: String,
		text: String,
		from: &'static str,
		subject: &'static str,
	}
	let _response = actix_web::client::Client::default()
		.post("https://api.mailgun.net/v3/journalism.blainehansen.me/messages")
		.header(http::header::AUTHORIZATION, MAILGUN_API_KEY.to_owned())
		.send_form(&MailgunForm {
			to: email,
			from: "Journalism Cooperative Validation <no-reply@journalism.blainehansen.me>",
			subject: "Journalism Cooperative - Validation Email",
			text: format!("Hello! It works!\nClick this link to validate your email:\nhttps://journalism.blainehansen.me/verify?t={}", verification_token),
		})
		.await
		.map_err(internal_error)?;

	Ok(actix_web::HttpResponse::NoContent().finish())
}


#[derive(Debug, Deserialize)]
struct VerifyEmailMessage {
	verification_token: String,
}
#[post("/verify")]
async fn verify(
	data: Json<VerifyEmailMessage>,
	pool: Pool,
) -> actix_web::Result<actix_web::HttpResponse> {
	let rows = sqlx::query!(
		r#"update subscription set verification_token = null where verification_token = $1"#,
		data.verification_token,
	)
		.execute(&**pool).await
		.map_err(internal_error)?
		.rows_affected();

	match rows {
		0 | 1 => Ok(actix_web::HttpResponse::NoContent().finish()),
		_ => Err(invalid_rows(rows).into()),
	}
}


// https://documentation.mailgun.com/en/latest/api-events.html#event-structure
// https://documentation.mailgun.com/en/latest/user_manual.html#webhooks
// #[derive(Deserialize)]
// #[serde(rename_all = "lowercase")]
// enum MailgunWebhookType {
// 	Unsubscribed,
// 	Complained,
// 	// Failed
// }

// #[derive(Deserialize)]
// struct MailgunEventMessage {
// 	signature: MailgunEventSignature,
// 	#[serde(rename = "event-data")]
// 	event_data: MailgunEventData,
// }

// #[derive(Deserialize)]
// struct MailgunEventSignature {
// 	timestamp: String,
// 	token: String,
// 	signature: String,
// }

// #[derive(Deserialize)]
// struct MailgunEventData {
// 	event: MailgunWebhookType,
// 	recipient: String,
// }

#[derive(Debug, Deserialize)]
struct UnsubscribeMessage {
	#[serde(deserialize_with = "from_base64")]
	email: String,
	unsubscribed_with: String,
}
#[post("/unsubscribe")]
async fn unsubscribe(
	data: Json<UnsubscribeMessage>,
	pool: Pool,
) -> actix_web::Result<actix_web::HttpResponse> {
	let rows = sqlx::query!(
		r#"update subscription set unsubscribed_with = $1 where email = $2::text"#,
		data.unsubscribed_with, data.email,
	)
		.execute(&**pool).await
		.map_err(internal_error)?
		.rows_affected();

	match rows {
		0 | 1 => Ok(actix_web::HttpResponse::NoContent().finish()),
		_ => Err(invalid_rows(rows).into()),
	}
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// std::thread::sleep(std::time::Duration::from_secs(5));

	assert!(MAILGUN_API_KEY.to_owned() != "");
	// #[allow(non_snake_case)]
	// let KEY_FILE = get_env("KEY_FILE")?;
	// #[allow(non_snake_case)]
	// let CERT_FILE = get_env("CERT_FILE")?;
	#[allow(non_snake_case)]
	let DATABASE_URL = get_env("DATABASE_URL")?;
	#[allow(non_snake_case)]
	let BIND_URL = get_env("BIND_URL")?;
	#[allow(non_snake_case)]
	let ALLOWED_ORIGIN = get_env("ALLOWED_ORIGIN")?;

	std::env::set_var("RUST_LOG", "actix_web=info,email_server=info");
	env_logger::init();

	// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
	// let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
	// builder.set_private_key_file("/usr/local/bin/key.pem", SslFiletype::PEM).unwrap();
	// builder.set_certificate_chain_file("/usr/local/bin/cert.pem").unwrap();

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&DATABASE_URL).await
		.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

	actix_web::HttpServer::new(move || {
		let app = actix_web::App::new()
			.data(pool.clone())
			// https://actix.rs/actix-web/actix_web/middleware/struct.Logger.html
			.wrap(actix_web::middleware::Logger::new(r#"time: %t, bytes: %b, ms: %T, "%r" ==> %s"#))
			.wrap(actix_cors::Cors::new()
				.allowed_origin(&ALLOWED_ORIGIN)
				.max_age(86400)
				.finish()
			);

		app
			.service(subscribe)
			.service(verify)
			.service(unsubscribe)
	})
		// .bind_openssl(BIND_URL, builder)?
		.bind(BIND_URL)?
		.run().await
}
