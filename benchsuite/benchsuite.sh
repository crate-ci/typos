#!/usr/bin/env bash
set -e

current_dir=`dirname $(readlink -f $0)`

base_dir="/tmp/benchsuite"
if [[ $# -ge 1 ]]; then
  base_dir=$1
fi
mkdir -p $base_dir
pushd $base_dir
base_dir=.


machine="$HOSTNAME"
if [[ $# -ge 2 ]]; then
  machine=$2
fi

current_day=`date +%Y-%m-%d`
report_prefix=$current_dir/runs/$current_day-$machine
report_path=$report_prefix.md
mkdir -p `dirname $report_path`

echo "" > $report_path
echo "# Spell Check Shootout" >> $report_path
echo "" >> $report_path
echo "These are the results as of $current_day" >> $report_path
echo "" >> $report_path
echo "Command:" >> $report_path
echo "\`\`\`bash" >> $report_path
echo "$ $0 $base_dir $machine" >> $report_path
echo "\`\`\`" >> $report_path
echo "" >> $report_path
echo "" >> $report_path

function print_tool() {
  local name=$1
  local version=$2
  local path=$3
  local output=$4
  if [[ -z $path ]]; then
    >&2 echo "Warning: $name uut is unavailable"
    echo "- $name: N/A" >> $output
  else
    echo "- $version" >> $output
  fi
}

echo "Spell checkers:" >> $report_path
rg_path=`$current_dir/uut/rg.sh path $base_dir`
rg_version=`$current_dir/uut/rg.sh version $base_dir`
print_tool "rg" "$rg_version" "$rg_path" "$report_path"
echo "  - Though not a spell checker, present to be a theoretical lower bound" >> $report_path

typos_path=`$current_dir/uut/typos.sh path $base_dir`
typos_version=`$current_dir/uut/typos.sh version $base_dir`
print_tool "typos" "$typos_version" "$typos_path" "$report_path"

misspell_go_path=`$current_dir/uut/misspell_go.sh path $base_dir`
misspell_go_version=`$current_dir/uut/misspell_go.sh version $base_dir`
print_tool "misspell_go" "$misspell_go_version" "$misspell_go_path" "$report_path"

codespell_path=`$current_dir/uut/codespell.sh path $base_dir`
codespell_version=`$current_dir/uut/codespell.sh version $base_dir`
print_tool "codespell" "$codespell_version" "$codespell_path" "$report_path"

scspell_path=`$current_dir/uut/scspell.sh path $base_dir`
scspell_version=`$current_dir/uut/scspell.sh version $base_dir`
print_tool "scspell" "$scspell_version" "$scspell_path" "$report_path"

echo "" >> $report_path

function bench_dir() {
  local name=$1
  local version=$2
  local path=$3
  local output=$4

  echo "## $name fixture" >> $output
  echo "" >> $output
  if [[ -z $path ]]; then
    >&2 echo "Warning: $name fixture is unavailable"
    echo "N/A" >> $output
  else
    echo "$name: $version" >> $output
    echo "" >> $output
    rg_command=""
    rg_j1_command=""
    if [[ ! -z $rg_path ]]; then
      rg_command="$rg_path bin $path"
      rg_j1_command="$rg_path --threads 1 bin $path"
    fi
    typos_command=""
    typos_ascii_command=""
    typos_j1_command=""
    if [[ ! -z $typos_path ]]; then
      typos_command="$typos_path $path"
      typos_ascii_command="$typos_path --no-unicode $path"
      typos_j1_command="$typos_path --threads 1 $path"
    fi
    misspell_go_command=""
    if [[ ! -z $misspell_go_path ]]; then
      misspell_go_command="$misspell_go_path $path"
    fi
    # Skipping scspell, doesn't work on directories
    codespell_command=""
    if [[ ! -z $codespell_path ]]; then
      codespell_command="$codespell_path $path"
    fi
    hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$rg_j1_command" "$typos_command" "$typos_ascii_command" "$typos_j1_command" "$misspell_go_command" "$codespell_command"
    cat $report_prefix-rg.md >> $output
  fi
  echo "" >> $output
}

function bench_file() {
  local name=$1
  local version=$2
  local path=$3
  local output=$4

  echo "## $name fixture" >> $output
  echo "" >> $output
  if [[ -z $path ]]; then
    >&2 echo "Warning: $name fixture is unavailable"
    echo "N/A" >> $output
  else
    echo "$name: $version" >> $output
    echo "" >> $output
    rg_command=""
    if [[ ! -z $rg_path ]]; then
      rg_command="$rg_path bin $path"
    fi
    typos_command=""
    typos_ascii_command=""
    if [[ ! -z $typos_path ]]; then
      typos_command="$typos_path $path"
      typos_ascii_command="$typos_path --no-unicode $path"
    fi
    misspell_go_command=""
    if [[ ! -z $misspell_go_path ]]; then
      misspell_go_command="$misspell_go_path $path"
    fi
    scspell_command=""
    if [[ ! -z $scspell_path ]]; then
      scspell_command="$scspell_path $path"
    fi
    codespell_command=""
    if [[ ! -z $codespell_path ]]; then
      codespell_command="$codespell_path $path"
    fi
    hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$typos_ascii_command" "$misspell_go_command" "$scspell_command" "$codespell_command"
    cat $report_prefix-rg.md >> $output
  fi
  echo "" >> $output
}

linux_clean_path=`$current_dir/fixtures/linux_clean.sh path $base_dir`
linux_clean_version=`$current_dir/fixtures/linux_clean.sh version $base_dir`
bench_dir "linux_clean" "$linux_clean_version" "$linux_clean_path" "$report_path"

linux_built_path=`$current_dir/fixtures/linux_built.sh path $base_dir`
linux_built_version=`$current_dir/fixtures/linux_built.sh version $base_dir`
bench_dir "linux_built" "$linux_built_version" "$linux_built_path" "$report_path"

ripgrep_clean_path=`$current_dir/fixtures/ripgrep_clean.sh path $base_dir`
ripgrep_clean_version=`$current_dir/fixtures/ripgrep_clean.sh version $base_dir`
bench_dir "ripgrep_clean" "$ripgrep_clean_version" "$ripgrep_clean_path" "$report_path"

ripgrep_built_path=`$current_dir/fixtures/ripgrep_built.sh path $base_dir`
ripgrep_built_version=`$current_dir/fixtures/ripgrep_built.sh version $base_dir`
bench_dir "ripgrep_built" "$ripgrep_built_version" "$ripgrep_built_path" "$report_path"

subtitles_en_path=`$current_dir/fixtures/subtitles_en.sh path $base_dir`
subtitles_en_version=`$current_dir/fixtures/subtitles_en.sh version $base_dir`
bench_file "subtitles_en" "$subtitles_en_version" "$subtitles_en_path" "$report_path"

subtitles_en_small_path=`$current_dir/fixtures/subtitles_en_small.sh path $base_dir`
subtitles_en_small_version=`$current_dir/fixtures/subtitles_en_small.sh version $base_dir`
bench_file "subtitles_en_small" "$subtitles_en_small_version" "$subtitles_en_small_path" "$report_path"

subtitles_ru_path=`$current_dir/fixtures/subtitles_ru.sh path $base_dir`
subtitles_ru_version=`$current_dir/fixtures/subtitles_ru.sh version $base_dir`
bench_file "subtitles_ru" "$subtitles_ru_version" "$subtitles_ru_path" "$report_path"

subtitles_ru_small_path=`$current_dir/fixtures/subtitles_ru_small.sh path $base_dir`
subtitles_ru_small_version=`$current_dir/fixtures/subtitles_ru_small.sh version $base_dir`
bench_file "subtitles_ru_small" "$subtitles_ru_smal_version" "$subtitles_ru_small_path" "$report_path"
