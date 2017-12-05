#!/bin/sh
set -e

find . \
  -maxdepth 1 \
  -type d \
  -name "day*" | sort | while read fname
do
    echo "*** $fname ***"
    cd $fname
    cargo test -q
    cargo run -q --release
    cd ..
done