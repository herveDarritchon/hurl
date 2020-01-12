from tests import app
from flask import redirect

@app.route('/redirect')
def redirectme():
    return redirect('http://redirectme')