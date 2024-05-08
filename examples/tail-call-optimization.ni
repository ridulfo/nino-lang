let increment:fn = (x:num, i:num):num => i ? {
    0 => x,
    increment(x+1, i-1)
};

print(increment(0, 20000));
