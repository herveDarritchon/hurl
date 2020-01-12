from flask import request
from tests import app

@app.route("/default-headers")
def default_headers():
    #print(request.headers)
    assert 'User-Agent' in request.headers
    assert 'Host' in request.headers
    assert int(request.headers['Content-Length']) == 0
    return ''