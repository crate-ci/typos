#!/bin/bash

set -eu

log() {
    echo -e "$1" >&2
}

CMD_NAME="typos"
TARGET=${INPUT_FILES:-"."}

if [[ -z $(ls ${TARGET} 2>/dev/null) ]]; then
    log "ERROR: Input files (${TARGET}) not found"
    exit 1
fi
if [[ -z $(which ${CMD_NAME} 2>/dev/null) ]]; then
    log "ERROR: 'typos' not found"
    exit 1
fi

COMMAND="${CMD_NAME} ${TARGET}"

# Ignore implicit configuration files
if [ "${INPUT_ISOLATED:-false}" == "true" ]; then
    COMMAND+=" --isolated"
fi

# Use a custom configuration file
if [[ -n "${INPUT_CONFIG:-}" ]]; then
    COMMAND+=" --config ${INPUT_CONFIG}"
fi

log "$ ${COMMAND}"
${COMMAND}
