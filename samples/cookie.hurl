GET http://localhost:5000/set-cookies
HTTP/1.1 200

GET http://localhost:5000/send-cookie1-value1
HTTP/1.1 200

GET http://localhost:5000/send-cookie1-value3
[Cookies]
Cookie1: value3
HTTP/1.1 200