first_numbers = (n) => {
    let fib_numbers = [0,1];
    while (fib_numbers.length < n ){
        fib_numbers.push(fib_numbers[fib_numbers.length-1] + fib_numbers[fib_numbers.length-2])
    }
    return fib_numbers;
}

if (require.main === module){
    console.log(first_numbers(20));
}

