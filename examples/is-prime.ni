let is_prime_helper:fn = (x:i32, dividend:i32, check_to:i32):bool => true ? {
    dividend>check_to => true,
    x mod dividend == 0 => false,
    is_prime_helper(x, dividend + 2, check_to)
};

let is_prime:fn = (x:i32):bool => x ? {
    1 => false,
    2 => true,
    is_prime_helper(x, 2, sqrt(x)+1)
};

let print_primes:fn = (current:i32, end:i32):i32 => true ? {
    current == end => 0,
    is_prime(current) => print_primes(print(current)+1 , end),
    print_primes(current + 1, end)
};

print(print_primes(10000, 10100));
