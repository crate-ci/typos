#!/usr/bin/env bash
set -e

UUT_DIR="rg"
RG_VERSION="11.0.1"
RG_TARGET="x86_64-unknown-linux-musl"

if [[ $# -eq 0 ]]; then
  exit 1
fi
command=$1

base_dir="/tmp/benchsuite"
if [[ $# -ge 2 ]]; then
  base_dir=$2
fi

root_dir="${base_dir}/$UUT_DIR"
out_file="$root_dir/rg"
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
    curl -L -o rg.tgz https://github.com/BurntSushi/ripgrep/releases/download/11.0.1/ripgrep-$RG_VERSION-$RG_TARGET.tar.gz
    tar -zxvf rg.tgz >> $log_path
    cp */rg .
    popd >> $log_path
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    echo "`$out_file --version`"
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
