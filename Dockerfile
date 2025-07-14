ARG TARGETPLATFORM

FROM --platform=linux/amd64 rust:alpine AS build
ENV HOME="/root"
WORKDIR $HOME

# build with script
RUN apk add alpine-sdk zig
RUN cargo install cargo-zigbuild

COPY build.sh Cargo.toml Cargo.lock dropprs.rs .
RUN chmod +x build.sh
RUN ./build.sh arm-unknown-linux-musleabihf
RUN ./build.sh armv7-unknown-linux-musleabihf
RUN ./build.sh aarch64-unknown-linux-musl
RUN ./build.sh x86_64-unknown-linux-musl

FROM alpine AS binary
ARG TARGETPLATFORM
COPY --from=build /root/target /root/target
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") export TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64") export TARGET="aarch64-unknown-linux-musl" ;; \
    "linux/arm/v7") export TARGET="armv7-unknown-linux-musleabihf" ;; \
    "linux/arm/v6") export TARGET="arm-unknown-linux-musleabihf" ;; \
    *) export TARGET="x86_64-unknown-linux-musl" ;; \
  esac && \
    cp /root/target/${TARGET}/release/dropprs /dropprs

FROM scratch
COPY --from=binary /dropprs /
ENTRYPOINT ["/dropprs"]