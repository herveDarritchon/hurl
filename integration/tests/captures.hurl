GET http://localhost:8000/captures

HTTP/1.0 200
[Captures]
param1: header header1


GET http://localhost:8000/captures-check
[QueryStringParams]
param1: {{param1}}


HTTP/1.0 200

