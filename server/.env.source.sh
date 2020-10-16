export $(grep -v '^#' server/.env | xargs -d '\n')
