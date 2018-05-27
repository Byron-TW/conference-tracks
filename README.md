## Goal

Solve the conference tracks problem the way I would like it, in Rust.
Try to be as idiomatic as possible, and consider the answer the main user
value to provide. Thus that and only that MUST be tested, anything else 
is 'extra', like actually testing for a few things the user can run into.

## Getting the answers

Run `make answers` if your `rust` installation is at least at v1.26.
If you have no `rust` but `docker`, run `make answers-in-docker`.

In any case, you can run all `make` targets using docker via `make interactive-developer-environment-in-docker`.
Please be warned that initial compilation takes a while.

## Features

* [x] shows correct answers
* [x] support for profiling
* [x] support for benchmarking
* [x] support for linting
* [x] interactive developer environment in docker

## Notes

* so far implementation of the solver was beyond me, and I used a ready-made one from the internet.
* there are rough edges about how memory is used
  * removing scheduled talks from the talks list is very inefficient.
  * a relatively big lookup table is allocated over and over again

## Benchmark Results

A total of 181 lines of code compile to a 481kb binary (stripped), which can process input with 130 items of input in 80ms.
It's not particularly fast actually, probably also due to inefficiencies shown in the notes.

A python implementation comes in at 181 lines of code, but processes the benchark data in 45ms!
