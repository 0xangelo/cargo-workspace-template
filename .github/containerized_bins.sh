#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"

ALL=$(cargo metadata --no-deps --format-version=1 | jq -r ".packages[] | select(.name == \"$1\") | .targets[] | select(.kind == [\"bin\"]) | .name")
unset BINS
for bin in $ALL
do
  grep -q "^FROM .* AS $bin\$" $REPO_ROOT/docker/Dockerfile && BINS="$BINS $bin"
done
echo $BINS | jq -R 'split(" ") | [.[] | select(length>0) | {bin: .}]'
