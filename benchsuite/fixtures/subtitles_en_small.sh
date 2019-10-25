#!/usr/bin/env bash
set -e

FIXTURE_DIR="subtitles_en"
SUBTITLES_NAME="OpenSubtitles2016.raw.en"
SUBTITLES_NAME_SMALL="OpenSubtitles2016.raw.en.small"
LINES=10000

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

current_dir=`dirname $(readlink -f $0)`
root_dir="${base_dir}/$FIXTURE_DIR"
out_file="$root_dir/$SUBTITLES_NAME_SMALL"
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
    in_file=`$current_dir/subtitles_en.sh download $base_dir`
    head -n $LINES $in_file > $out_file
    shasum $out_file > $out_file.sha
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
