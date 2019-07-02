#!/usr/bin/env bash
# Pre-reqs:
# - git
set -e

FIXTURE_DIR="ripgrep_clean"
REPO_TAG="11.0.1"
REPO_URL="git://github.com/BurntSushi/ripgrep"

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
    mkdir -p ${root_dir}
    echo "Downloading $FIXTURE_DIR" >> ${log_path}
    git clone --depth 1 $REPO_URL --branch $REPO_TAG $root_dir >> ${log_path}
  fi
}

function version() {
  if [[ -e $out_file ]]; then
    pushd $root_dir >> ${log_path}
    echo "rg `git rev-parse HEAD`"
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
