from tests import app

@app.route("/assert-header")
def assert_header():
    return 'Hello World!'

