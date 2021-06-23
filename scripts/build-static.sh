#!/bin/bash

set -e

docker run --rm -it \
    -e SQLX_OFFLINE='true' \
    -v "$(pwd)":"/home/rust/src" ekidd/rust-musl-builder:1.51.0 \
    cargo build --release --target x86_64-unknown-linux-musl

strip target/x86_64-unknown-linux-musl/release/shortest-url
