set -e

rm -rf target
docker build -t blainehansen/journalism-cooperative:latest .
docker push blainehansen/journalism-cooperative:latest
