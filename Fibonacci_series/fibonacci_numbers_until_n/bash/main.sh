#!/bin/bash

function numbers_until_n(){
    fib_numbers=(0 1)
    sum=1
    while [ $sum -lt $1 ]; do
        fib_numbers+=($sum)
        length=${#fib_numbers[@]}
        last=${fib_numbers[$length-1]}
        penultimate=${fib_numbers[$length-2]}
        sum=$(( $last + $penultimate ))
    done
    echo ${fib_numbers[@]}
}

numbers_until_n 5000