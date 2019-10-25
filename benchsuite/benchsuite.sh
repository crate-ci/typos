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

linux_clean_path=`$current_dir/fixtures/linux_clean.sh path $base_dir`
linux_clean_version=`$current_dir/fixtures/linux_clean.sh version $base_dir`

linux_built_path=`$current_dir/fixtures/linux_built.sh path $base_dir`
linux_built_version=`$current_dir/fixtures/linux_built.sh version $base_dir`

ripgrep_clean_path=`$current_dir/fixtures/ripgrep_clean.sh path $base_dir`
ripgrep_clean_version=`$current_dir/fixtures/ripgrep_clean.sh version $base_dir`

ripgrep_built_path=`$current_dir/fixtures/ripgrep_built.sh path $base_dir`
ripgrep_built_version=`$current_dir/fixtures/ripgrep_built.sh version $base_dir`

subtitles_en_path=`$current_dir/fixtures/subtitles_en.sh path $base_dir`
subtitles_en_version=`$current_dir/fixtures/subtitles_en.sh version $base_dir`

subtitles_en_small_path=`$current_dir/fixtures/subtitles_en_small.sh path $base_dir`
subtitles_en_small_version=`$current_dir/fixtures/subtitles_en_small.sh version $base_dir`

subtitles_ru_path=`$current_dir/fixtures/subtitles_ru.sh path $base_dir`
subtitles_ru_version=`$current_dir/fixtures/subtitles_ru.sh version $base_dir`

subtitles_ru_small_path=`$current_dir/fixtures/subtitles_ru_small.sh path $base_dir`
subtitles_ru_small_version=`$current_dir/fixtures/subtitles_ru_small.sh version $base_dir`
echo "" >> $report_path


echo "Spell checkers:" >> $report_path
rg_path=`$current_dir/uut/rg.sh path $base_dir`
rg_version=`$current_dir/uut/rg.sh version $base_dir`
if [[ -z $rg_path ]]; then
  >&2 echo "Warning: rg uut is unavailable"
  echo "- rg: N/A" >> $report_path
else
  echo "- $rg_version" >> $report_path
fi
echo "  - Though not a spell checker, present to be a theoretical lower bound" >> $report_path

typos_path=`$current_dir/uut/typos.sh path $base_dir`
typos_version=`$current_dir/uut/typos.sh version $base_dir`
if [[ -z $typos_path ]]; then
  >&2 echo "Warning: typos uut is unavailable"
  echo "- typos: N/A" >> $report_path
else
  echo "- $typos_version" >> $report_path
fi

misspell_rs_path=`$current_dir/uut/misspell_rs.sh path $base_dir`
misspell_rs_version=`$current_dir/uut/misspell_rs.sh version $base_dir`
if [[ -z $misspell_rs_path ]]; then
  >&2 echo "Warning: misspell_rs uut is unavailable"
  echo "- misspell_rs: N/A" >> $report_path
else
  echo "- $misspell_rs_version" >> $report_path
fi

misspell_go_path=`$current_dir/uut/misspell_go.sh path $base_dir`
misspell_go_version=`$current_dir/uut/misspell_go.sh version $base_dir`
if [[ -z $misspell_go_path ]]; then
  >&2 echo "Warning: misspell_go uut is unavailable"
  echo "- misspell_go: N/A" >> $report_path
else
  echo "- $misspell_go_version" >> $report_path
fi

codespell_path=`$current_dir/uut/codespell.sh path $base_dir`
codespell_version=`$current_dir/uut/codespell.sh version $base_dir`
if [[ -z $codespell_path ]]; then
  >&2 echo "Warning: codespell uut is unavailable"
  echo "- codespell: N/A" >> $report_path
else
  echo "- $codespell_version" >> $report_path
fi

scspell_path=`$current_dir/uut/scspell.sh path $base_dir`
scspell_version=`$current_dir/uut/scspell.sh version $base_dir`
if [[ -z $scspell_path ]]; then
  >&2 echo "Warning: scspell uut is unavailable"
  echo "- scspell: N/A" >> $report_path
else
  echo "- $scspell_version" >> $report_path
fi
echo "" >> $report_path


echo "## linux_clean fixture" >> $report_path
echo "" >> $report_path
if [[ -z $linux_clean_path ]]; then
  >&2 echo "Warning: linux_clean fixture is unavailable"
  echo "N/A" >> $report_path
else
  echo "linux_clean: $linux_clean_version" >> $report_path
  echo "" >> $report_path
  rg_command=""
  if [[ ! -z $rg_path ]]; then
    rg_command="$rg_path bin $linux_clean_path"
  fi
  typos_command=""
  if [[ ! -z $typos_path ]]; then
    typos_command="$typos_path $linux_clean_path"
  fi
  misspell_rs_command=""
  if [[ ! -z $misspell_rs_path ]]; then
    misspell_rs_command="$misspell_rs_path $linux_clean_path"
  fi
  misspell_go_command=""
  if [[ ! -z $misspell_go_path ]]; then
    misspell_go_command="$misspell_go_path $linux_clean_path"
  fi
  # Skipping scspell, doesn't work on directories
  codespell_command=""
  if [[ ! -z $codespell_path ]]; then
    codespell_command="$codespell_path $linux_clean_path"
  fi
  hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$misspell_rs_command" "$misspell_go_command" "$codespell_command"
  cat $report_prefix-rg.md >> $report_path
fi
echo "" >> $report_path


echo "## linux_built fixture" >> $report_path
echo "" >> $report_path
if [[ -z $linux_built_path ]]; then
  >&2 echo "Warning: linux_built fixture is unavailable"
  echo "N/A" >> $report_path
else
  echo "linux_built: $linux_built_version" >> $report_path
  echo "" >> $report_path
  rg_command=""
  if [[ ! -z $rg_path ]]; then
    rg_command="$rg_path bin $linux_built_path"
  fi
  typos_command=""
  if [[ ! -z $typos_path ]]; then
    typos_command="$typos_path $linux_built_path"
  fi
  misspell_rs_command=""
  if [[ ! -z $misspell_rs_path ]]; then
    misspell_rs_command="$misspell_rs_path $linux_built_path"
  fi
  misspell_go_command=""
  if [[ ! -z $misspell_go_path ]]; then
    misspell_go_command="$misspell_go_path $linux_built_path"
  fi
  # Skipping scspell, doesn't work on directories
  codespell_command=""
  if [[ ! -z $codespell_path ]]; then
    codespell_command="$codespell_path $linux_built_path"
  fi
  hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$misspell_rs_command" "$misspell_go_command" "$codespell_command"
  cat $report_prefix-rg.md >> $report_path
fi
echo "" >> $report_path


if [[ -z $subtitles_en_path ]]; then
  >&2 echo "Warning: subtitles_en fixture is unavailable"
fi


if [[ -z $subtitles_en_small_path ]]; then
  >&2 echo "Warning: subtitles_en_small fixture is unavailable"
fi


echo "## subtitles_ru_small fixture" >> $report_path
echo "" >> $report_path
if [[ -z $subtitles_ru_small_path ]]; then
  >&2 echo "Warning: subtitles_ru_small fixture is unavailable"
  echo "N/A" >> $report_path
else
  echo "subtitles_ru_small: $subtitles_ru_small_version" >> $report_path
  echo "" >> $report_path
  rg_command=""
  if [[ ! -z $rg_path ]]; then
    rg_command="$rg_path bin $subtitles_ru_small_path"
  fi
  typos_command=""
  if [[ ! -z $typos_path ]]; then
    typos_command="$typos_path $subtitles_ru_small_path"
  fi
  misspell_rs_command=""
  if [[ ! -z $misspell_rs_path ]]; then
    misspell_rs_command="$misspell_rs_path $subtitles_ru_small_path"
  fi
  misspell_go_command=""
  if [[ ! -z $misspell_go_path ]]; then
    misspell_go_command="$misspell_go_path $subtitles_ru_small_path"
  fi
  scspell_command=""
  if [[ ! -z $scspell_path ]]; then
    scspell_command="$scspell_path $subtitles_ru_small_path"
  fi
  codespell_command=""
  if [[ ! -z $codespell_path ]]; then
    codespell_command="$codespell_path $subtitles_ru_small_path"
  fi
  hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$misspell_rs_command" "$misspell_go_command" "$scspell_command" "$codespell_command"
  cat $report_prefix-rg.md >> $report_path
fi
echo "" >> $report_path


echo "## ripgrep_clean fixture" >> $report_path
echo "" >> $report_path
if [[ -z $ripgrep_clean_path ]]; then
  >&2 echo "Warning: ripgrep_clean fixture is unavailable"
  echo "N/A" >> $report_path
else
  echo "ripgrep_clean: $ripgrep_clean_version" >> $report_path
  echo "" >> $report_path
  rg_command=""
  if [[ ! -z $rg_path ]]; then
    rg_command="$rg_path bin $ripgrep_clean_path"
  fi
  typos_command=""
  if [[ ! -z $typos_path ]]; then
    typos_command="$typos_path $ripgrep_clean_path"
  fi
  misspell_rs_command=""
  if [[ ! -z $misspell_rs_path ]]; then
    misspell_rs_command="$misspell_rs_path $ripgrep_clean_path"
  fi
  misspell_go_command=""
  if [[ ! -z $misspell_go_path ]]; then
    misspell_go_command="$misspell_go_path $ripgrep_clean_path"
  fi
  # Skipping scspell, doesn't work on directories
  codespell_command=""
  if [[ ! -z $codespell_path ]]; then
    codespell_command="$codespell_path $ripgrep_clean_path"
  fi
  hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$misspell_rs_command" "$misspell_go_command" "$codespell_command"
  cat $report_prefix-rg.md >> $report_path
fi
echo "" >> $report_path


echo "## ripgrep_built fixture" >> $report_path
echo "" >> $report_path
if [[ -z $ripgrep_built_path ]]; then
  >&2 echo "Warning: ripgrep_built fixture is unavailable"
  echo "N/A" >> $report_path
else
  echo "ripgrep_built: $ripgrep_built_version" >> $report_path
  echo "" >> $report_path
  rg_command=""
  if [[ ! -z $rg_path ]]; then
    rg_command="$rg_path bin $ripgrep_built_path"
  fi
  typos_command=""
  if [[ ! -z $typos_path ]]; then
    typos_command="$typos_path $ripgrep_built_path"
  fi
  misspell_rs_command=""
  if [[ ! -z $misspell_rs_path ]]; then
    misspell_rs_command="$misspell_rs_path $ripgrep_built_path"
  fi
  misspell_go_command=""
  if [[ ! -z $misspell_go_path ]]; then
    misspell_go_command="$misspell_go_path $ripgrep_built_path"
  fi
  # Skipping scspell, doesn't work on directories
  codespell_command=""
  if [[ ! -z $codespell_path ]]; then
    codespell_command="$codespell_path $ripgrep_built_path"
  fi
  hyperfine --warmup 1 -i --export-json $report_prefix-rg.json --export-markdown $report_prefix-rg.md "$rg_command" "$typos_command" "$misspell_rs_command" "$misspell_go_command" "$codespell_command"
  cat $report_prefix-rg.md >> $report_path
fi
echo "" >> $report_path
