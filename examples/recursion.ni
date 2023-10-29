let factorial:fn = (n:i32):i32 => n ? {
    0 => 1,
    n * factorial(n - 1)
};
print(factorial(5))