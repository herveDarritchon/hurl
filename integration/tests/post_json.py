from flask import request
from tests import app

@app.route('/post-json', methods=['POST'])
def post_json():
    s = request.data.decode("utf-8")
    assert s == '''{
    "name": "Bob",
    "password": "secret"
}'''
    return ''

