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

let print_primes:fn = (current:i32, end:i32):i32 => true ? {
    current == end => 0,
    is_prime(current) => print_primes(print(current)+1 , end),
    print_primes(current + 1, end)
};

print(print_primes(10000, 10100));
