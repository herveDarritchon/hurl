GET http://localhost:5000/hello

HTTP/1.1 200
[Asserts]
body equals "Hello World!"
body startsWith "Hello"
body contains "llo"
body matches "Hello [a-zA-Z]+!"