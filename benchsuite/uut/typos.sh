#!/usr/bin/env bash
set -e

UUT_DIR="typos"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

current_dir=`dirname $(readlink -f $0)`
root_dir="${base_dir}/$UUT_DIR"
out_file="$root_dir/bin/typos"
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

    cargo --version > $root_dir/cargo.txt
    cargo install --path `realpath $current_dir/../..` --root $root_dir
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    echo "`$out_file --version` w/ `cat $root_dir/cargo.txt`"
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
