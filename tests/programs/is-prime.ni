# This is the recursive function that tries to divide x by all odd numbers
let is_prime_helper:fn = (x:num, dividend:num, check_to:num):bool => true ? {
    dividend>check_to => true,
    x mod dividend == 0 => false,
    is_prime_helper(x, dividend + 2, check_to)
};

# Main is_prime function
let is_prime:fn = (x:num):bool => true ? {
    x == 1 => false,
    x == 2 => true,
    x mod 2 == 0 => false,
    is_prime_helper(x, 3, sqrt(x)+1)
};

let is_prime_23:num = is_prime(23);
let is_prime_100:num = is_prime(100);
let is_prime_10000189:num = is_prime(10000189);
