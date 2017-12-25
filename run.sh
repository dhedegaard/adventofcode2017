#!/bin/sh
set -e
set -x

find . \
  -maxdepth 1 \
  -type d \
  -name "day*" | sort | while read fname
do
    echo "*** $fname ***"
    cd $fname
    cargo test -q --release
    cargo run -q --release
    cd ..
done