#!/bin/bash
set -e
cd "$(dirname "$0")"

# Static Analysis
./hurl_echo tests/*.hurl tests_error_lint/*.hurl
./lint.sh tests_error_lint/*.hurl
./generate_html



# Dynamic Run
docker rm -f nginx flask || true

python -V

which python36

pip install Flask==1.1.1
python server.py&
sleep 2

./run.sh tests/*.hurl tests_error_parser/*.hurl



