from tests import app
from flask import Response


@app.route("/assert-match")
def assert_match():
    return Response('''{
  "success":false,
  "errors":[{"id":"error1"},{"id":"error2"}], 
  "warnings": []
}''', mimetype='application/json')
