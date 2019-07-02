#!/usr/bin/env bash
set -e

UUT_DIR="scspell"
SCSPELL_VERSION="2.2"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

root_dir="${base_dir}/$UUT_DIR"
bin_dir=$root_dir/bin
out_file="$bin_dir/scspell"
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
    mkdir -p ${base_dir}
    echo "Downloading $UUT_DIR" >> ${log_path}

    python3 -m venv $root_dir >> $log_path

    # My version of Ubuntu is using 8.1.1 and unsure if I want to touch it.
    $bin_dir/pip install -U pip==9.0.3 >> $log_path
    $bin_dir/pip install -U scspell3k==$SCSPELL_VERSION >> $log_path
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    echo "`$out_file --version` w/ `$bin_dir/python3 --version`"
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
