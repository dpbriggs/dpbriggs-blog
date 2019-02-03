#!/bin/bash

kill $(pidof target/release/dpbriggs-blog) &&
cargo run --release
