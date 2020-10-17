set -e

# sign up
PGPASSWORD=$SERVER_POSTGRES_PASSWORD psql -h localhost -U rust_server_user database -c "insert into subscription (email, verification_token) values
	('dude@gmail.com', '5lhI4yC6t2Vn/T/5oKdrDM8urmKOJoj+UeFuXfvlcZmYoBoKO5FzQl0bKFyZNgaGqVGCoQXi53hDDHc/EMbXvw==') on conflict (email) do nothing"
curl -k "$BASE_URL:$API_PORT/subscribe" -H "Content-Type: application/json" \
	-d '{ "email": "dude@gmail.com" }'


# verify
# PGPASSWORD=$SERVER_POSTGRES_PASSWORD psql -h localhost -U rust_server_user database -c "update subscription set verification_token = null where verification_token = '5lhI4yC6t2Vn/T/5oKdrDM8urmKOJoj+UeFuXfvlcZmYoBoKO5FzQl0bKFyZNgaGqVGCoQXi53hDDHc/EMbXvw=='"
curl -k $BASE_URL:$API_PORT/verify -H "Content-Type: application/json" \
	-d '{ "verification_token": "5lhI4yC6t2Vn/T/5oKdrDM8urmKOJoj+UeFuXfvlcZmYoBoKO5FzQl0bKFyZNgaGqVGCoQXi53hDDHc/EMbXvw==" }'


# unsubscribe
PGPASSWORD=$POSTGRES_PASSWORD psql -h localhost -U $POSTGRES_USER database -c "insert into unsubscribe_token (token, description) values ('6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A==', 'stuff')"

# PGPASSWORD=$SERVER_POSTGRES_PASSWORD psql -h localhost -U rust_server_user database -c "update subscription set unsubscribed_with = '6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A==' where email = 'dude@gmail.com'"
curl -k "$BASE_URL:$API_PORT/unsubscribe" -H "Content-Type: application/json" \
	-d '{ "email": "ZHVkZUBnbWFpbC5jb20=", "unsubscribed_with": "6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A==" }'
