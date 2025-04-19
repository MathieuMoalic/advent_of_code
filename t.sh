#!/usr/bin/env bash

for i in {02..25}; do
        cp src/year2015/day01.rs "src/year2015/day${i}.rs"
done

for i in {16..25}; do
        rm -r "src/year20${i}"
        cp -r src/year2015/ "src/year20${i}"
done
