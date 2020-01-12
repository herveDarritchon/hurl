#!/bin/bash

set -e
set -u

ci/check_tag
ci/create_tarballs
ci/upload.sh "$VERSION" "target/tarballs/hurl-$VERSION-x86_64-linux.tar.gz"

