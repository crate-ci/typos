
# Spell Check Shootout

These are the results as of 2019-10-29

Command:
```bash
$ ./benchsuite.sh . Nightblood
```


Spell checkers:
- ripgrep 11.0.1 (rev 973de50c9e) -SIMD -AVX (compiled) +SIMD +AVX (runtime)
  - Though not a spell checker, present to be a theoretical lower bound
- typos-cli 0.1.2 w/ cargo 1.38.0 (23ef9a4ef 2019-08-20)
- misspell 0.2.0 w/ cargo 1.38.0 (23ef9a4ef 2019-08-20)
- misspell_go 0.3.4
- codespell 1.15.0 w/ Python 3.5.2
- scspell 2.2 w/ Python 3.5.2

## linux_clean fixture

N/A

## linux_built fixture

N/A

## ripgrep_clean fixture

ripgrep_clean: rg 973de50c9ef451da2cfcdfa86f2b2711d8d6ff48

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./ripgrep_clean` | 25.8 ± 4.8 | 21.0…53.8 |
| `./typos/bin/typos ./ripgrep_clean` | 145.1 ± 6.3 | 134.1…157.2 |
| `./misspell_rs/bin/misspell ./ripgrep_clean` | 131.3 ± 4.5 | 124.9…142.4 |
| `./misspell_go/bin/misspell ./ripgrep_clean` | 167.1 ± 6.1 | 160.9…184.6 |
| `./codespell/bin/codespell ./ripgrep_clean` | 560.9 ± 9.3 | 546.8…575.0 |

## ripgrep_built fixture

ripgrep_built: rg 973de50c9ef451da2cfcdfa86f2b2711d8d6ff48

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./ripgrep_built` | 30.0 ± 9.1 | 22.3…71.2 |
| `./typos/bin/typos ./ripgrep_built` | 154.9 ± 9.7 | 141.4…169.4 |
| `./misspell_rs/bin/misspell ./ripgrep_built` | 150.0 ± 4.0 | 145.6…158.7 |
| `./misspell_go/bin/misspell ./ripgrep_built` | 282.4 ± 8.0 | 265.9…292.8 |
| `./codespell/bin/codespell ./ripgrep_built` | 841.7 ± 10.1 | 828.8…862.0 |

## subtitles_ru_small fixture

subtitles_ru_small: 

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 10.9 ± 3.1 | 7.8…26.4 |
| `./typos/bin/typos ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 84.9 ± 6.6 | 75.0…100.5 |
| `./misspell_rs/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 41.5 ± 3.0 | 37.3…53.8 |
| `./misspell_go/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 49.3 ± 3.8 | 44.6…65.5 |
| `./scspell/bin/scspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 333.4 ± 4.7 | 321.9…338.5 |
| `./codespell/bin/codespell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 360.9 ± 9.9 | 346.6…380.0 |

