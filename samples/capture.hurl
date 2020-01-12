GET http://localhost:5000/headers

HTTP/1.1 200
[Captures]
param1: header Header1


GET http://localhost:5000/querystring-params
[QueryStringParams]
param1: {{param1}}
param2: ""

HTTP/1.1 200

