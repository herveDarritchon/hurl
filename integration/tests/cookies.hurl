GET http://localhost:8000/cookies/send-cookie1-value1
[Cookies]
cookie1: value1
HTTP/1.0 200


GET http://localhost:8000/cookies/set-cookie2-value1
HTTP/1.0 200
[Asserts]
cookie cookie2 equals "value1"


GET http://localhost:8000/cookies/send-cookie2-value1
HTTP/1.0 200


GET http://localhost:8000/cookies/send-cookie2-value2
[Cookies]
cookie2: value2
HTTP/1.0 200