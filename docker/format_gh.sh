#!/bin/bash

set -eu

grep '"type":"typo"' |
  jq --sort-keys --raw-output '"::warning file=\(.path),line=\(.line_num),col=\(.byte_offset)::\"\(.typo)\" should be \"" + (.corrections // [] | join("\" or \"") + "\".")' |
  while IFS= read -r line; do
    echo "$line"
  done
