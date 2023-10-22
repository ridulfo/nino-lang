let inc:fn = (x:i32):i32=>x+1;
let dec:fn = (x:i32):i32=>x-1;

let a:i32 = inc(0);
let b:i32 = dec(0);
let c:i32 = a+b;
print(a);
print(b);
print(c);