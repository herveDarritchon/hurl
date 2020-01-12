#
# Test body response with line ending LF and CRLF.
# We receive the text body "\nline1\nline2\rline3\r\n\n" and assert
# the body encoded in base64
# printf "\nline1\nline2\rline3\r\n\n" | base64

GET http://localhost:5000/end-of-lines
HTTP/1.1 200
base64,CmxpbmUxCmxpbmUyDWxpbmUzDQoK;
