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

# Function to print all the prints from current to end
let print_primes:fn = (current:num, end:num):num => true ? {
    current == end => 0,
    is_prime(current) => print_primes(print(current)+1 , end),
    print_primes(current + 1, end)
};

print_primes(10000000, 10000200);
