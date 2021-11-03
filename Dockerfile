FROM ubuntu:20.04
ARG VERSION=1.2.1
ENV VERSION=${VERSION}
RUN apt-get update && apt-get install -y wget
RUN wget https://github.com/crate-ci/typos/releases/download/v${VERSION}/typos-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    tar -xzvf typos-v${VERSION}-x86_64-unknown-linux-musl.tar.gz && \
    mv typos /usr/local/bin
ENTRYPOINT ["/usr/local/bin/typos"]
