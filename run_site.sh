#!/bin/bash

EXISTING_INSTANCE="$(pidof target/release/dpbriggs-blog)"

if [ ! -z $EXISTING_INSTANCE ]; then
    kill $EXISTING_INSTANCE
fi

BIN_NAME="dpbriggs-blog"

if [ -f "dpbriggs-blog" ]; then
    ./$BIN_NAME
    exit $?
fi

if [ -f "Cargo.toml" ]; then
   cargo run --release
   exit $?
fi

echo "Could not find dpbriggs-blog executable or Cargo.toml"

