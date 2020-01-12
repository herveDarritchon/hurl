GET http://localhost:8000/assert-json
HTTP/1.0 200
[Asserts]
jsonpath $.success equals false
jsonpath $.errors countEquals 2
jsonpath $.warnings countEquals 0
jsonpath $.toto not exists
jsonpath $.warnings exists
