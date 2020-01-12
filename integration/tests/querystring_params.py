from tests import app
from flask import request

@app.route("/querystring-params")
def querystring_params():
    assert request.args.get('param1') == 'value1'
    assert request.args.get('param2') == ''
    return ''