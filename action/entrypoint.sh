#!/bin/bash

set -eu

SOURCE_DIR="$(dirname -- ${BASH_SOURCE[0]:-$0})";

log() {
    echo -e "$1" >&2
}

_DEFAULT_INSTALL_DIR=${HOME}/bin
_INSTALL_DIR=${INSTALL_DIR:-${_DEFAULT_INSTALL_DIR}}
CMD_NAME="typos"
COMMAND="${_INSTALL_DIR}/${CMD_NAME}"

TARGET=${INPUT_FILES:-"."}
if [[ -z $(ls ${TARGET} 2>/dev/null) ]]; then
    log "ERROR: Input files (${TARGET}) not found"
    exit 1
fi


if [[ ! -x ${COMMAND} ]]; then
    VERSION=1.29.0
    if [[ "$(uname -m)" == "arm64" ]]; then
        ARCH="aarch64"
    else
        ARCH="x86_64"
    fi
    UNAME=$(uname -s)
    if [[ "$UNAME" == "Darwin" ]]; then
        TARGET_FILE="${ARCH}-apple-darwin"
        FILE_EXT="tar.gz"
    elif [[ "$UNAME" == CYGWIN* || "$UNAME" == MINGW* || "$UNAME" == MSYS* ]] ; then
        TARGET_FILE="${ARCH}-pc-windows-msvc"
        FILE_EXT="zip"
    else
        TARGET_FILE="${ARCH}-unknown-linux-musl"
        FILE_EXT="tar.gz"
    fi
    FILE_NAME="typos-v${VERSION}-${TARGET_FILE}.${FILE_EXT}"
    log "Downloading 'typos' v${VERSION}"
    wget --progress=dot:mega "https://github.com/crate-ci/typos/releases/download/v${VERSION}/${FILE_NAME}"
    mkdir -p ${_INSTALL_DIR}
    if [[ "$FILE_EXT" == "zip" ]]; then
        unzip -o "${FILE_NAME}" -d ${_INSTALL_DIR} ${CMD_NAME}.exe
    else
        tar -xzvf "${FILE_NAME}" -C ${_INSTALL_DIR} ./${CMD_NAME}
    fi
    rm "${FILE_NAME}"
fi
log "jq: $(jq --version)"

ARGS="${TARGET}"

# Ignore implicit configuration files
if [ "${INPUT_ISOLATED:-false}" == "true" ]; then
    ARGS+=" --isolated"
fi

# Write changes to the repository
if [ "${INPUT_WRITE_CHANGES:-false}" == "true" ]; then
    ARGS+=" --write-changes"
fi

# Use a custom configuration file
if [[ -n "${INPUT_CONFIG:-}" ]]; then
    ARGS+=" --config ${INPUT_CONFIG}"
fi

log "$ ${COMMAND} ${ARGS}"
${COMMAND} ${ARGS} --format json | ${SOURCE_DIR}/format_gh.sh || true
${COMMAND} ${ARGS}
