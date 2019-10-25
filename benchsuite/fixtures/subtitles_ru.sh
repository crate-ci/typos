#!/usr/bin/env bash
set -e

FIXTURE_DIR="subtitles_ru"
SUBTITLES_NAME="OpenSubtitles2016.raw.ru"
SUBTITLES_NAME_GZ="${SUBTITLES_NAME}.gz"
SUBTITLES_URL="https://object.pouta.csc.fi/OPUS-OpenSubtitles/v2016/mono/ru.txt.gz"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

root_dir="${base_dir}/$FIXTURE_DIR"
out_file="$root_dir/$SUBTITLES_NAME"
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
    shasum $SUBTITLES_NAME > $SUBTITLES_NAME.sha
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
