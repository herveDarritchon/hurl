#!/bin/bash

set -e
set -u

API_URL="https://api.github.com"
REPO_NAME=hurl
repo_url="$API_URL/repos/fabricereix/$REPO_NAME"
auth_header="Authorization: token $GITHUB_API_TOKEN"
asset_files=$*

for asset_file in $asset_files; do
  echo "Uploading asset file $asset_file"
  if [ ! -f "$asset_file" ]; then
     echo "does not exist!"
     exit 1
  fi
done



