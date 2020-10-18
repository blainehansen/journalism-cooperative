export $(grep -v '^#' $1 | xargs -d '\n')

# export DATABASE_URL="postgresql://$rust_server_user:$SERVER_POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB"
export DATABASE_URL="postgresql://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST/$POSTGRES_DB"
export BIND_URL=0.0.0.0:$API_PORT
export ALLOWED_ORIGIN=$BASE_URL
