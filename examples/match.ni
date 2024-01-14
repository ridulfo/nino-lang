let inc:fn = (x:num):num=>x+1;

let b:num = inc(0) ? {
    0 => inc(1),
    1 => inc(2),
};

print(b);
