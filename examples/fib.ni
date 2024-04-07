# Simple fib
let simple_fib:fn = (n:num):num => n ? {
	0 => 0,
	1 => 1,
	simple_fib(n-1) + simple_fib(n-2)
};

# Correct fib
let correct_fib:fn = (n:num):num => true ? {
	n<0 => print("n must be greater or equal to 0"),
	n==0 => 0,
	n==1 => 1,
	correct_fib(n-1) + correct_fib(n-2)
};


# Tail-call optimized fib
let fib_helper:fn = (n:num, fib1:num, fib2:num):num => n ? {
	0 => fib1,
	fib_helper(n-1, fib2, fib1+fib2)
};
let tail_fib:fn = (n:num):num => fib_helper(n, 0, 1);
