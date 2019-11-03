
# Spell Check Shootout

These are the results as of 2019-11-02

Command:
```bash
$ ./benchsuite/benchsuite.sh . Nightblood
```


Spell checkers:
- ripgrep 11.0.1 (rev 973de50c9e) -SIMD -AVX (compiled) +SIMD +AVX (runtime)
  - Though not a spell checker, present to be a theoretical lower bound
- typos-cli 0.1.4 w/ cargo 1.38.0 (23ef9a4ef 2019-08-20)
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
| `./rg/rg bin ./ripgrep_clean` | 25.2 ± 3.0 | 20.0…37.8 |
| `./typos/bin/typos ./ripgrep_clean` | 124.3 ± 4.1 | 117.2…133.5 |
| `./misspell_rs/bin/misspell ./ripgrep_clean` | 127.7 ± 3.1 | 122.0…132.7 |
| `./misspell_go/bin/misspell ./ripgrep_clean` | 176.3 ± 4.3 | 170.2…184.7 |
| `./codespell/bin/codespell ./ripgrep_clean` | 647.2 ± 31.8 | 573.3…677.5 |

## ripgrep_built fixture

ripgrep_built: rg 973de50c9ef451da2cfcdfa86f2b2711d8d6ff48

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./ripgrep_built` | 34.1 ± 4.2 | 27.4…48.9 |
| `./typos/bin/typos ./ripgrep_built` | 145.5 ± 4.7 | 138.6…151.9 |
| `./misspell_rs/bin/misspell ./ripgrep_built` | 146.3 ± 5.8 | 135.5…162.7 |
| `./misspell_go/bin/misspell ./ripgrep_built` | 301.0 ± 10.0 | 289.9…323.2 |
| `./codespell/bin/codespell ./ripgrep_built` | 861.6 ± 13.6 | 839.8…886.8 |

## subtitles_ru_small fixture

subtitles_ru_small: 

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 10.5 ± 1.8 | 6.8…18.1 |
| `./typos/bin/typos ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 78.7 ± 7.0 | 66.7…97.0 |
| `./misspell_rs/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 40.6 ± 2.5 | 33.8…46.6 |
| `./misspell_go/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 48.6 ± 3.5 | 41.2…58.4 |
| `./scspell/bin/scspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 332.5 ± 12.6 | 312.6…351.7 |
| `./codespell/bin/codespell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 370.8 ± 7.4 | 363.2…384.4 |

