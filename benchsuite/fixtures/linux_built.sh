#!/usr/bin/env bash
# Pre-reqs:
# - git
# - able to build a Linux kernel
#   - apt install libelf-dev, bc
set -e

FIXTURE_DIR="linux_built"

function cpucount() {
  echo `grep -c ^processor /proc/cpuinfo`
}

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
out_file="$root_dir"
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
    echo "Downloading $FIXTURE_DIR" >> ${log_path}
    in_file=`$current_dir/linux_clean.sh download $base_dir`
    cp -R $in_file $out_file
    # We want to build the kernel because the process of building it produces
    # a lot of junk in the repository that a search tool probably shouldn't
    # touch.
    pushd $root_dir >> ${log_path}
    make defconfig >> ${log_path}
    make -j $(cpucount) >> ${log_path}
    popd >> ${log_path}
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    pushd $root_dir >> ${log_path}
    echo "linux `git rev-parse HEAD`"
    popd >> ${log_path}
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
