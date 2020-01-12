GET http://localhost:5000/utf8

HTTP/1.1 200
[Asserts]
xpath normalize-space(//data) equals "café"
xpath normalize-space(//data) equals "caf\u00e9"