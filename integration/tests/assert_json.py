from tests import app
from flask import Response

@app.route("/assert-json")
def assert_json():
    return Response('''{
  "success":false,
  "errors":[{"id":"error1"},{"id":"error2"}], 
  "warnings": []
}''', mimetype='application/json')