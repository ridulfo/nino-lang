let increment:fn = (x:i32, i:i32):i32 => i ? {
    0 => x,
    increment(x + 1, i - 1)
};

print(increment(0, 20000));