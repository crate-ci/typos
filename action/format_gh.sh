#!/bin/bash

set -eu

grep '"type":"typo"' | while IFS= read -r typo; do
  original_path="$(echo "$typo" | jq --raw-output '.path')"
  relative_path="$(realpath --relative-to="$GITHUB_WORKSPACE" "$original_path")"
  echo "$typo" | jq --arg relative_path "$relative_path" --raw-output \
    '"::warning file=\($relative_path),line=\(.line_num),col=\(.byte_offset + 1)::\"\(.typo)\" should be \"" + (.corrections // [] | join("\" or \"") + "\".")'
done
