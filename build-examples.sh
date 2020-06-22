#!/usr/bin/env bash

build_sample() {
    name=$1
    dir=samples/${name}
    cat ${dir}/input.csv | cargo run ${dir}/ysv.yaml > ${dir}/output.csv

}

for sample_name in $(ls samples); do
    build_sample ${sample_name}
done
