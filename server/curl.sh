curl -k https://localhost:5050/subscribe -v -H "Content-Type: application/json" \
	-d '{ "email": "dude.com" }'


# curl -k https://localhost:5050/verify -v -H "Content-Type: application/json" \
# 	-d '{ "validation_token": "-kAdWOSYUz2AeTD25xAifM5mZT-N4H26w5gxUkt_sZdTVpj9obXfAlRkGl_FUseWY-X0FjWurU7GRUl_ouYwhA==" }'


# curl -k https://localhost:5050/unsubscribe -v -H "Content-Type: application/json" \
# 	-d '{ "email": "ZHVkZUBnbWFpbC5jb20=", "unsubscribed_with": "6eOplSNiFgbAKhvQL//Kyvm+qXUgbMDHm1ZouUaBm6lzUndNI36DhSJuoiLrrGdVXFEIa8DpI9DviImYaSxk7A==" }'
