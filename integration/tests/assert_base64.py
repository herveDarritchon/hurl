from tests import app
from flask import Response

@app.route("/assert-base64")
def assert_base64():
    return '\nline1\nline2\rline3\r\n\n'