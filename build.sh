#!/bin/sh

case $TARGETPLATFORM in
  linux/arm/v6)
    TARGET=arm-unknown-linux-musleabihf
    ;;
  linux/arm/v7)
    TARGET=armv7-unknown-linux-musleabihf
    ;;
  linux/arm64)
    TARGET=aarch64-unknown-linux-musl
    ;;
  linux/amd64)
    TARGET=x86_64-unknown-linux-musl
    ;;
  *)
    TARGET=x86_64-unknown-linux-musl
    ;;
esac

# rustup
rustup target add "$TARGET"
cargo zigbuild --release --target "$TARGET"
mv target/"$TARGET"/release/dropprs /dropprs