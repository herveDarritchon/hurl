from flask import request
from tests import app

@app.route('/post-multilines', methods=['POST'])
def post_multilines():
    s = request.data.decode("utf-8")
    assert s == "".join([v + "\n" for v in ["name,age", "bob,10", "bill,22"]])
    return ''
