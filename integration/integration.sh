#!/bin/bash
set -e
cd "$(dirname "$0")"

# Static Analysis
./hurl_echo tests/*.hurl tests_error_lint/*.hurl
./lint.sh tests_error_lint/*.hurl
./generate_html

# Dynamic
./run.sh tests/*.hurl tests_error_parser/*.hurl



