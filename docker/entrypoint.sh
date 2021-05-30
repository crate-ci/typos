#!/bin/bash

set -e

printf "Found files in workspace:\n"
ls

printf "\nLooking for typos installed...\n"
which typos

COMMAND="typos"

# Show the _typos.toml file
if [ -f "_typos.toml" ]; then
    echo "_typos.toml:"
    cat _typos.toml
    echo
fi

# Ignore implicit configuration files
if [ "${INPUT_ISOLATED}" == "true" ]; then
    COMMAND="${COMMAND} --isolated"
fi


# Use a custom configuration file
if [ ! -z "${INPUT_CONFIG}" ]; then

    # It must exist
    if [ ! -f "${INPUT_CONFIG}" ]; then
        printf "${INPUT_CONFIG} does not exist.\n"
        exit 1;
    else
        # Show the custom config to the user
        printf "Custom config:\n"
        cat "${INPUT_CONFIG}"    
        echo
    fi
    COMMAND="${COMMAND} --config ${INPUT_CONFIG}"
fi

# Files are technically optional
if [ ! -z "${INPUT_FILES}" ]; then
    COMMAND="${COMMAND} ${INPUT_FILES}"
fi

echo "Command: "
echo "${COMMAND}"
echo

${COMMAND}
retval=$?

if [[ "${retval}" -eq 0 ]]; then
   printf "No spelling mistakes found! 🎉️\n"
else
   printf "Spelling mistakes found! 😱️\n"
   exit $retval;
fi
