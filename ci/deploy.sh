#!/bin/bash

set -e
set -u

ci/check_tag "$VERSION"
ci/create_tarballs target/tarballs
ci/upload.sh "$VERSION" "target/tarballs/hurl-$VERSION-x86_64-linux.tar.gz"

