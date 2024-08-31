#!/bin/bash

function first_numbers(){
    fib_numbers=(0 1)

    while [ ${#fib_numbers[@]} -lt $1 ]; do
        length=${#fib_numbers[@]}
        last=${fib_numbers[$length-1]}
        penultimate=${fib_numbers[$length-2]}
        fib_numbers+=( $(($last+$penultimate)) )
    done
    echo ${fib_numbers[@]}
    
}

first_20=$(first_numbers 20)
echo $first_20
