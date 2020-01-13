from flask import request, make_response
from tests import app


@app.route("/cookies/send-cookie1-value1")
def send_cookie1_value1():
    assert request.cookies['cookie1'] == 'value1'
    return ''


@app.route("/cookies/send-cookie2-value1")
def send_cookie2_value1():
    assert request.cookies['cookie2'] == 'value1'
    return ''


@app.route("/cookies/send-cookie2-value2")
def send_cookie2_value2():
    assert request.cookies['cookie2'] == 'value2'
    return ''


@app.route("/cookies/set-cookie2-value1")
def set_cookie2_value1():
    resp = make_response()
    resp.set_cookie('cookie2', 'value1')
    return resp


@app.route("/cookies/delete-cookie2")
def delete_cookie2():
    resp = make_response()
    resp.set_cookie('cookie2', 'value1', max_age=0)
    return resp

@app.route("/cookies/assert-that-cookie2-is-not-set")
def assert_that_cookie2_is_not_set():
    assert'cookie2' not in request.cookies
    return ''