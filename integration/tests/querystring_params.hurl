GET http://localhost:8000/querystring-params
[QueryStringParams]
param1: value1
param2: ""
param3: a=b

HTTP/1.0 200


# same version as raw
GET http://localhost:8000/querystring-params?param1=value1&param2=&param3=a%3db

HTTP/1.0 200


