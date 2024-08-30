fn first_n_fibonacci_numbers(n:usize)-> Vec<i128>{
    let mut fib_numbers = vec![0,1];
    let mut length = fib_numbers.len();
    while  length< n{
        let sum = fib_numbers[length-1] + fib_numbers[length-2];
        fib_numbers.push(sum);
        length = length + 1; 
    }
    return fib_numbers

}
fn main() {
    let fib_numbers = first_n_fibonacci_numbers(20);
    println!("{:#?}",&fib_numbers);
    print!("{}",&fib_numbers.len())

}
