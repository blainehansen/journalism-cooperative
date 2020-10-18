set -e

cargo sqlx prepare
docker build -t blainehansen/journalism-cooperative:latest .
docker push blainehansen/journalism-cooperative:latest
