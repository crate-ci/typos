# syntax=docker/dockerfile:1.10
# go to https://hub.docker.com/r/docker/dockerfile to see the latest version of the syntax

# Stage 1: Build the typos binary
FROM rust:1.81.0-slim-bookworm AS builder

# Install musl-tools for static linking
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        liblz4-tool \
        musl-tools \
        xz-utils \
    && \
    rm -rf /var/lib/apt/lists/*

# some targets were not used in the end because rust package is not working with them
#     x86_64-pc-windows-msvc \

RUN rustup target add \
    aarch64-apple-darwin \
    aarch64-unknown-linux-musl \
    aarch64-unknown-linux-musl \
    x86_64-apple-darwin \
    x86_64-unknown-linux-musl \
    && :

# Set the working directory
WORKDIR /usr/src/typos

# Copy the source code into the container
COPY . .

# Set build arguments
ARG TARGETPLATFORM
ARG BIN_NAME=typos

# Determine the Rust target based on the platform
# fingers crossed this build will just work
# in case I need more platforms - https://github.com/containerd/containerd/blob/90cd777a6c8c92c105625ba086e2e67a0c32d7ed/platforms/platforms.go#L88-L94
#     elif [ "${TARGETPLATFORM}" = "windows/amd64" ]; then \
#         RUST_TARGET="x86_64-pc-windows-msvc"; \
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/typos/target \
    set -xeEu \
    && \
    ARM_PLATFORMS='linux/arm/v6 linux/arm/v7 linux/arm64/v8 linux/arm64' \
    && \
    if [ "${TARGETPLATFORM}" = "darwin/arm64" ]; then \
        RUST_TARGET="aarch64-apple-darwin"; \
    elif printf '%s\n' ${ARM_PLATFORMS} | grep -Fxq "${TARGETPLATFORM}" ; then \
        RUST_TARGET="aarch64-unknown-linux-musl"; \
    elif [ "${TARGETPLATFORM}" = "darwin/amd64" ]; then \
        RUST_TARGET="x86_64-apple-darwin"; \
    elif [ "${TARGETPLATFORM}" = "linux/amd64" ]; then \
        RUST_TARGET="x86_64-unknown-linux-musl"; \
    else \
        echo "Unsupported TARGETPLATFORM: ${TARGETPLATFORM}"; \
        exit 1; \
    fi \
    && \
    echo "Building for ${RUST_TARGET}" \
    && \
    cargo build \
        --release \
        --verbose \
        --target ${RUST_TARGET} \
    && \
    cp target/${RUST_TARGET}/release/${BIN_NAME} /usr/src/${BIN_NAME}/${BIN_NAME}

# Stage 2: Create the final image
FROM scratch

# Set build arguments
ARG BIN_NAME=typos

# Copy the statically linked binary from the builder stage
COPY --from=builder /usr/src/typos/${BIN_NAME} /${BIN_NAME}

# Set the entrypoint to the typos binary
# This was done to make the default run not scan the whole container for typos
WORKDIR /workdir
ENTRYPOINT ["/typos"]