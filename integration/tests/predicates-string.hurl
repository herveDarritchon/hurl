GET http://localhost:8000/predicates-string

HTTP/1.0 200
[Asserts]
body equals "Hello World!"
body startsWith "Hello"
body contains "llo"
body matches "Hello [a-zA-Z]+!"