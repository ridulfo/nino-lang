let a:i32 = 3;

let b:i32 = a ? {
    2 => 3,
    3 => 4,
    4 => 5,
};

let c:i32 = b - 1;

print(c);