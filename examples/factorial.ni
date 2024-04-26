let factorial_helper:fn = (n:num, acc:num):num => n ? {
	0 => acc,
	factorial(n-1, n*acc)
};

let factorial:fn = (n:num):num => (n>0) ? {
	true => factorial_helper(n-1, n),
	1
};

print(factorial(0));
print(factorial(1));
print(factorial(2));
