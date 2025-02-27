#!/usr/bin/env bash

cargo build --examples --profile=release-with-debug
perf record --call-graph dwarf target/release/examples/book1_1
perf report

