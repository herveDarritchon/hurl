from tests import app

@app.route('/error-assert/http-version')
def http_version():
    return ''


