fn fibonacci_numbers(n:i128) -> Vec<i128>{
    let mut fib_numbers = vec![0,1];
    let mut sum = 1;
    

    while sum < n{
        fib_numbers.push(sum);
        let length = fib_numbers.len();
        sum  = fib_numbers[length-1] + fib_numbers[length-2];
         
    }
    fib_numbers
}

fn main() {
    println!("{:#?}", fibonacci_numbers(5000));
}
