let factorial:fn = (n:i32):i32 => n ? {
    0 => 1,
    n * factorial(n - 1)
};
print(factorial(10));

let fib:fn = (n:i32):i32 => n ? {
    0 => 0,
    1 => 1,
    fib(n - 1) + fib(n - 2)
};

print(fib(24));