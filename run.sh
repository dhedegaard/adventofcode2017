#!/bin/sh
set -e

find . \
  -type d \
  -maxdepth 1 \
  -name "day*" | sort | while read fname
do
    echo "*** $fname ***"
    cd $fname
    cargo test -q
    cargo run -q --release
    cd ..
done