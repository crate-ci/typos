FROM rust:bullseye as builder
WORKDIR /usr/src/typos
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/typos /usr/local/bin/typos
CMD ["typos"]
