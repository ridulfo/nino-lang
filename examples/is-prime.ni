let is_prime_helper:fn = (x:i32, i:i32):bool => true ? {
    x==i => true,
    x mod i == 0 => false,
    is_prime_helper(x, i + 1)
};

let is_prime:fn = (x:i32):bool => x ? {
    1 => false,
    2 => true,
    is_prime_helper(x, 2)
};

print(is_prime(1));
print(is_prime(2));
print(is_prime(3));
print(is_prime(4));
print(is_prime(5));
print(is_prime(6));
print(is_prime(7));
