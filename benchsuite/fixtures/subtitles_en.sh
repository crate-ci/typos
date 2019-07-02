#!/usr/bin/env bash
# Pre-reqs:
# - gunzip
# - curl
set -e

FIXTURE_DIR="subtitles_en"
SUBTITLES_NAME="OpenSubtitles2016.raw.en"
SUBTITLES_NAME_SAMPLE="$SUBTITLES_NAME.sample"
SUBTITLES_NAME_GZ="${SUBTITLES_NAME}.gz"
SUBTITLES_URL="https://object.pouta.csc.fi/OPUS-OpenSubtitles/v2016/mono/en.txt.gz"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

root_dir="${base_dir}/$FIXTURE_DIR"
out_file="$root_dir/$SUBTITLES_NAME_SAMPLE"
log_path="${base_dir}/$FIXTURE_DIR.log"

function path() {
  if [[ -e $out_file ]]; then
    echo $out_file
  fi
}

function clear() {
  rm -Rf ${root_dir} ${log_path}
}

function download() {
  if [[ ! -e $out_file ]]; then
    mkdir -p ${root_dir}
    echo "Downloading $FIXTURE_DIR" >> ${log_path}
    pushd $root_dir >> ${log_path}
    curl -L $SUBTITLES_URL -o $SUBTITLES_NAME_GZ
    gunzip $SUBTITLES_NAME_GZ
    # Get a sample roughly the same size as the Russian corpus so that
    # benchmarks finish in a reasonable time.
    head -n 32722372 $SUBTITLES_NAME > $SUBTITLES_NAME_SAMPLE
    shasum $SUBTITLES_NAME_SAMPLE > $SUBTITLES_NAME_SAMPLE.sha
    popd >> ${log_path}
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    echo "`basename $out_file` `cat $out_file.sha | cut -d " " -f 1`"
  fi
}

case $command in
  path)
    echo $(path)
    ;;
  clear)
    echo $(clear)
    ;;
  version)
    echo $(version)
    ;;
  download)
    download
    echo $(path)
    ;;
  *)
    >&2 echo "Invalid command: $command"
    exit 1
    ;;
esac
