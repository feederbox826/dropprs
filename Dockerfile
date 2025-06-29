FROM rust:alpine AS build
ENV HOME="/root"
WORKDIR $HOME

# Install rust
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
  "linux/arm64") echo "aarch64-unknown-linux-musl" > rust_target.txt ;; \
  "linux/amd64") echo "x86_64-unknown-linux-musl" > rust_target.txt ;; \
  *) exit 1 ;; \
  esac

# Update rustup whenever we bump the rust version
RUN rustup target add $(cat rust_target.txt)

COPY . ./

# Build
RUN cargo build --target $(cat rust_target.txt) --release
RUN cp target/$(cat rust_target.txt)/release/dropprs /dropprs
RUN strip --strip-all /dropprs

FROM scratch
COPY --from=build /dropprs /
ENTRYPOINT ["/dropprs"]