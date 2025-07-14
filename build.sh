#!/bin/sh

TARGET=$1

# rustup
rustup target add "$TARGET"
cargo zigbuild --release --target "$TARGET"