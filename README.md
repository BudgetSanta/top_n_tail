# Top 'n' Tail :carrot:
[![Crates Badge]][Crates] [![Docs Badge]][Docs] [![License Badge]][License]

A CLI Utility to extract text from files or `stdin`

# Usage

Reading from `stdin`
```shell 
$ tnt 5 15 < seq 1 20
$ seq 1 20 | tnt 5 15
```

Reading from file
```shell
$ seq 1 20 > my_seq.txt && tnt 5 15 -f my_seq.txt
```

# Performance

Generating test data using `seq` I made 9,999,999 lines to test with.
I emulated the behaviour using `tail` and `head` and then ran it against `tnt`.

```shell
$ seq 9999999 | time tail -9999998 | head -9999997 > /dev/null
tail -9999998  9.62s user 8.38s system 93% cpu 19.188 total
head -9999997 > /dev/null  3.12s user 5.18s system 48% cpu 17.220 total
```

_The total time to extract took about 19 seconds_


```shell
$ seq 9999999 | time tnt 1 9999997 > /dev/null
tnt 1 9999997 > /dev/null  0.36s user 0.21s system 17% cpu 3.286 total
```

_The same output took about 3 seconds with `tnt`_

[Crates]: https://crates.io/crates/top_n_tail
[Crates Badge]: https://img.shields.io/crates/v/top_n_tail

[Docs]: https://docs.rs/top_n_tail
[Docs Badge]: https://docs.rs/top_n_tail/badge.svg

[License]: https://spdx.org/licenses/MIT.html
[License Badge]: https://img.shields.io/badge/License-MIT-blue.svg