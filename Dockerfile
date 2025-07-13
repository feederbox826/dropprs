ARG TARGETPLATFORM
FROM --platform=linux/amd64 rust:alpine AS build
ENV HOME="/root"
WORKDIR $HOME

# build with script
RUN apk add alpine-sdk zig
RUN cargo install cargo-zigbuild

COPY . ./
RUN chmod +x build.sh
RUN ./build.sh

FROM scratch
COPY --from=build /dropprs /
ENTRYPOINT ["/dropprs"]