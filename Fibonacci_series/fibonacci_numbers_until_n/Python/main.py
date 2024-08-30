def numbers_before_n(n):
    fib_numbers = [0,1]
    sum = 1
    
    while sum <= n:
        fib_numbers.append(sum)
        sum = fib_numbers[len(fib_numbers)-1] + fib_numbers[len(fib_numbers)-2]
    return fib_numbers

if __name__ == "__main__":
    print(numbers_before_n(5000))