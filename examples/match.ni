let inc:fn = (x:i32):i32=>x+1;

let b:i32 = inc(0) ? {
    0 => inc(1),
    1 => inc(2),
};

print(b);