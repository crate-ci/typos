
# Spell Check Shootout

These are the results as of 2019-10-24

Command:
```bash
$ ./benchsuite.sh . Nightblood
```


Spell checkers:
- ripgrep 11.0.1 (rev 973de50c9e) -SIMD -AVX (compiled) +SIMD +AVX (runtime)
- typos-cli 0.1.0 w/ cargo 1.38.0 (23ef9a4ef 2019-08-20)
- misspell 0.2.0 w/ cargo 1.38.0 (23ef9a4ef 2019-08-20)
- misspell_go 0.3.4
- codespell 1.15.0 w/ Python 3.5.2
- scspell 2.2 w/ Python 3.5.2

## linux_clean fixture

N/A

## linux_built fixture

N/A

## subtitles_ru_small fixture

subtitles_ru_small: OpenSubtitles2016.raw.ru.small c4549d470463cae24b3dbb1efd138192242c0853

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 7.6 ± 1.2 | 5.7…12.0 |
| `./typos/bin/typos ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 61.2 ± 4.1 | 52.4…70.1 |
| `./misspell_rs/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 33.5 ± 2.3 | 30.2…40.0 |
| `./misspell_go/bin/misspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 40.2 ± 2.6 | 34.1…46.1 |
| `./scspell/bin/scspell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 281.5 ± 3.8 | 276.8…289.3 |
| `./codespell/bin/codespell ./subtitles_ru/OpenSubtitles2016.raw.ru.small` | 311.8 ± 5.8 | 299.8…321.8 |

## ripgrep_clean fixture

ripgrep_clean: rg 973de50c9ef451da2cfcdfa86f2b2711d8d6ff48

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./ripgrep_clean` | 27.6 ± 5.1 | 20.6…38.1 |
| `./typos/bin/typos ./ripgrep_clean` | 168.0 ± 11.4 | 145.4…182.4 |
| `./misspell_rs/bin/misspell ./ripgrep_clean` | 145.4 ± 4.1 | 136.1…153.0 |
| `./misspell_go/bin/misspell ./ripgrep_clean` | 214.8 ± 7.8 | 193.4…226.5 |
| `./codespell/bin/codespell ./ripgrep_clean` | 651.0 ± 15.1 | 628.9…682.0 |

## ripgrep_built fixture

ripgrep_built: rg 973de50c9ef451da2cfcdfa86f2b2711d8d6ff48

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./rg/rg bin ./ripgrep_built` | 32.5 ± 4.9 | 26.1…41.5 |
| `./typos/bin/typos ./ripgrep_built` | 174.1 ± 5.9 | 163.8…187.5 |
| `./misspell_rs/bin/misspell ./ripgrep_built` | 143.8 ± 5.0 | 137.2…161.0 |
| `./misspell_go/bin/misspell ./ripgrep_built` | 278.6 ± 8.1 | 266.7…291.6 |
| `./codespell/bin/codespell ./ripgrep_built` | 840.5 ± 11.2 | 819.4…853.0 |

