#!/bin/bash

set -eu

log() {
    echo -e "$1" >&2
}

CMD_NAME="typos"
TARGET=${INPUT_FILES:-"."}
if [[ -n "${GITHUB_BASE_REF:-}" ]]; then
    git config --global --add safe.directory /github/workspace
    if git rev-parse --verify master 2>/dev/null ; then
        log "Limiting checks to ${GITHUB_BASE_REF}...HEAD"
        TARGET=$(git diff ${GITHUB_BASE_REF}...HEAD --name-only --diff-filter=AM -- ${TARGET})
    else
        log "WARN: Not limiting checks to ${GITHUB_BASE_REF}...HEAD, ${GITHUB_BASE_REF} is not available"
    fi
fi

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
${COMMAND} --format json |
  jq --sort-keys --raw-output '"::warning file=\(.path),line=\(.line_num),col=\(.byte_offset)::\"\(.typo)\" should be \"" + (.corrections // [] | join("\" or \"") + "\".")' |
  while IFS= read -r line; do
    echo "$line"
  done || true
${COMMAND}
