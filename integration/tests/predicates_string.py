from flask import request
from tests import app

@app.route('/predicates-string')
def predicates_string():
    return 'Hello World!'

