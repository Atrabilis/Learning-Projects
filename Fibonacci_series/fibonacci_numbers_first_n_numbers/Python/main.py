def first_n_numbers(n):
    fib_numbers = [0,1]
    while len(fib_numbers) <=n:
        fib_numbers.append(fib_numbers[len(fib_numbers)-1] + fib_numbers[len(fib_numbers)-2])

    return fib_numbers

if __name__ == "__main__":
    print(first_n_numbers(19))