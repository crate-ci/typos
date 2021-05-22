#!/usr/bin/env bash
set -e

UUT_DIR="misspell_go"
MISSPELL_GO_VERSION="0.3.4"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

root_dir="${base_dir}/$UUT_DIR"
out_file="$root_dir/bin/misspell"
log_path="${base_dir}/$UUT_DIR.log"

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
    echo "Downloading $UUT_DIR" >> ${log_path}

    pushd $root_dir >> $log_path
    curl -L -o ./install-misspell.sh https://git.io/misspell
    sh ./install-misspell.sh >> $log_path
    popd >> $log_path
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    echo "$UUT_DIR `$out_file -v`"
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
