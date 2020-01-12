GET http://127.0.0.1:5000/json
HTTP/1.1 200
[Asserts]
jsonpath $.success equals false
jsonpath $.errors equals 5
