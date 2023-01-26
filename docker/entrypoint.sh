#!/bin/bash

set -eu

SOURCE_DIR="$(dirname -- ${BASH_SOURCE[0]:-$0})";

log() {
    echo -e "$1" >&2
}

CMD_NAME="typos"
TARGET=${INPUT_FILES:-"."}
if [[ -n "${GITHUB_BASE_REF:-}" ]]; then
    BASE_REF=HEAD~  # HACK: GITHUB_BASE_REF is failing the `--verify` but `HEAD~ should be the same for pull requests
    git config --global --add safe.directory "$PWD"
    if git rev-parse --verify ${BASE_REF} 2>/dev/null ; then
        log "Limiting checks to ${GITHUB_BASE_REF}...HEAD"
        TARGET=$(git diff ${BASE_REF}...HEAD --name-only --diff-filter=AM -- ${TARGET})
        if [[ -z "${TARGET:-}" ]]; then
                log "INPUT_FILES are unchanged"
                exit 0
        fi
    else
        log "WARN: Not limiting checks to ${BASE_REF}...HEAD, ${GITHUB_BASE_REF} is not available"
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
log "typos: $(typos --version)"
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

log "$ ${CMD_NAME} ${ARGS}"
${CMD_NAME} ${ARGS} --format json | ${SOURCE_DIR}/format_gh.sh || true
${CMD_NAME} ${ARGS}
