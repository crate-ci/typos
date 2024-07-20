ARG DEBIAN_DIST=bullseye

FROM rust:${DEBIAN_DIST} as builder
WORKDIR /usr/src/typos
COPY . .
RUN cargo install --path ./crates/typos-cli

FROM debian:${DEBIAN_DIST}-slim
COPY --from=builder /usr/local/cargo/bin/typos /usr/local/bin/typos
ENTRYPOINT ["typos"]
CMD ["--help"]
