from tests import app
from flask import make_response, request


@app.route('/captures')
def captures():
    resp = make_response()
    resp.headers['Header1'] = 'value1'
    return resp


@app.route('/captures-check')
def captures_check():
    assert request.args.get('param1') == 'value1'
    return ''

