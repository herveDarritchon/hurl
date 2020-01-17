GET http://localhost:8000/assert-match
HTTP/1.0 200
[Asserts]
body matches ".*"
#body matches ".*warnings:\\s?\\[\\].*"
