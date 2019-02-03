#!/bin/bash

EXISTING_INSTANCE="$(pidof target/release/dpbriggs-blog)"

if [ ! -z $EXISTING_INSTANCE ]; then
    kill $EXISTING_INSTANCE
fi

cargo run --release
