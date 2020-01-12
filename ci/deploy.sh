#!/bin/bash

set -e
set -u

echo check git tag
TAG=$(git tag --points-at HEAD |tr -d '\n')
if [ "$TAG" == "" ]; then
        echo "Tag is not set"
        exit 0
fi

if [ "$TAG" != "$VERSION" ]; then
        echo "Tag '$TAG' does not match version '$VERSION'"
        exit 1
fi
echo tag matches version

ci/create_tarballs target/tarballs
ci/upload.sh "$VERSION" "target/tarballs/hurl-$VERSION-x86_64-linux.tar.gz"



