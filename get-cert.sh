openssl s_client -showcerts -connect plus.three.ie:443 < /dev/null | openssl x509 -outform DER > key.der
openssl x509 -inform DER -in key.der -out key.pem
