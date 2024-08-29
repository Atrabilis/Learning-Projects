fn fibonacci_numbers(n:i128) -> Vec<i128>{
    let mut fib_numbers = vec![0,1];
    let mut sum = 0;
    

    while sum <= n{
        let length = fib_numbers.len();
        sum  = fib_numbers[length-1] + fib_numbers[length-2];
        fib_numbers.push(sum) 
    }
    fib_numbers
}

fn main() {
    println!("{:#?}", fibonacci_numbers(100000));
}
