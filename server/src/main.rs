// use actix_web::{get, web::{self, Path}, Responder};
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

// macro_rules! query_one {
// 	($query: expr, $($args: expr),+) => {
// 		//
// 	};
// }

fn generate_random_token() -> String {
	use rand::prelude::*;
	let mut rng = rand::thread_rng();
	let mut buf: [u8; 64] = [0; 64];
	// TODO safer to use try_fill
	rng.fill(&mut buf);
	base64_encode(&buf[..])
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
		Err(e) => {
			log::info!("invalid email: {}", e);
			return Ok(actix_web::HttpResponse::BadRequest().finish());
		},
	};
	let verification_token = generate_random_token();

	let rows = sqlx::query!(
		r#"
			insert into subscription (email, verification_token) values ($1::text, $2)
			on conflict (email) do nothing
		"#,
		data.email, verification_token,
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
		.post("https://api.mailgun.net/v3/crowdsell.io/messages")
		.header(http::header::AUTHORIZATION, MAILGUN_API_KEY.to_owned())
		.send_form(&MailgunForm {
			// TODO real addresses
			to: "faichenshing@gmail.com".into(),
			from: "Journalism Cooperative Validation <no-reply@crowdsell.io>",
			subject: "Journalism Cooperative - Validation Email",
			// TODO real url
			text: format!("Hello! It works!\nClick this link to validate your email:\nhttps://example.com/validate?t={}", verification_token),
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

	std::env::set_var("RUST_LOG", "actix_web=info,journalism_cooperative=info");
	env_logger::init();

	use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
	let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
	builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
	builder.set_certificate_chain_file("cert.pem").unwrap();

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&get_env("DATABASE_URL")?).await
		.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

	actix_web::HttpServer::new(move || {
		let app = actix_web::App::new()
			.data(pool.clone())
			// https://actix.rs/actix-web/actix_web/middleware/struct.Logger.html
			// .wrap(actix_web::middleware::Logger::default())
			.wrap(actix_web::middleware::Logger::new(r#"time: %t, bytes: %b, ms: %T, "%r" ==> %s"#))
			.wrap(actix_cors::Cors::new()
				.allowed_origin("http://localhost:8080")
				.max_age(86400)
				.finish()
			);

		app
			.service(subscribe)
			.service(verify)
			.service(unsubscribe)
	})
		.bind_openssl("0.0.0.0:5050", builder)?
		.run().await
}