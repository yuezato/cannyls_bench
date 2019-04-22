#!/bin/sh

for i in {1..9}; do
    rm -f bench1.lusf;
    let r=$i*10000;
    ./target/debug/cannyls_bench -m storage -b bench1 -j 1Mb -d 1Gb -f bench1.lusf -r $r
done
