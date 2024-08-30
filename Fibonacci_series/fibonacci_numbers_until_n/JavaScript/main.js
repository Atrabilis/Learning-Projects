numbers_before_n = (n) => {
    fib_numbers = [0,1];
    sum = 1;
    while (sum < n){
        fib_numbers.push(sum)
        sum = fib_numbers[fib_numbers.length -1] + fib_numbers[fib_numbers.length -2]
    }
    return fib_numbers;
}

if(require.main === module){
    console.log(numbers_before_n(5000))
}
