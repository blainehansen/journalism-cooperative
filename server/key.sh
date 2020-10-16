openssl req -x509 -newkey rsa:4096 -nodes -keyout server/key.pem -out server/cert.pem -days 365 -subj '/CN=localhost'
