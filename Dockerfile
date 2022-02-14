FROM rust:1.58.1 as builder
WORKDIR /usr/src/typos
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/typos /usr/local/bin/typos
CMD ["typos"]
