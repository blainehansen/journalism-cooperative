create extension citext;

-- \set rust_server_password `echo $SERVER_POSTGRES_PASSWORD | tr -d "[:space:]"`
-- create role rust_server_user login password :'rust_server_password';

-- \set node_client_password `echo $CLIENT_POSTGRES_PASSWORD | tr -d "[:space:]"`
-- create role node_client_user login password :'node_client_password';

create table unsubscribe_token (
	token text primary key check (character_length(token) = 88),
	description text not null
);

create table subscription (
	email citext primary key check (email ~* '^.+@.+\..+$'),
	verification_token text unique check (character_length(verification_token) = 88),
	unsubscribed_with text references unsubscribe_token(token)
);

-- TODO create index on verification_token

create function is_unsubscribed(subscription) returns boolean as $$
  select $1.unsubscribed_with is not null;
$$ language sql immutable;

create function is_verified(subscription) returns boolean as $$
  select $1.verification_token is null;
$$ language sql immutable;


-- alter table subscription enable row level security;

-- grant select (verification_token) on table subscription to rust_server_user;
-- create policy rust_select_email on subscription for select to rust_server_user
-- 	using (true);

-- grant insert (email, site_name, verification_token) on table subscription to rust_server_user;
-- create policy rust_insert_email on subscription for insert to rust_server_user
-- 	with check (verification_token is not null);

-- grant update (verification_token) on table subscription to rust_server_user;
-- create policy rust_verify_email on subscription for update to rust_server_user
-- 	-- using (verification_token is not null)
-- 	using (true)
-- 	with check (verification_token is null);

-- grant select (unsubscribed_with, email, site_name) on table subscription to rust_server_user;
-- grant update (unsubscribed_with) on table subscription to rust_server_user;
-- create policy rust_unsubscribe_email on subscription for update to rust_server_user
-- 	using (true)
-- 	with check (unsubscribed_with is not null);

-- -- grant select * on table emails to node_client_user;
-- -- grant select * on table unsubscribe_tokens to node_client_user;