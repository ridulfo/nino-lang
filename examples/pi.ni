let polarity:fn = (x:num):num => true ? {
	n mod 2 == 0 => 1,
	-1
};

let pi_helper:fn = (n:num, n_max:num, acc:num):num => (n>n_max) ? {
	true => acc,
	false => pi_helper(n+1, n_max, acc + 4 * polarity(n) / (2 * n + 1))
};

let pi:fn = (n:num):num => pi_helper(0, n, 0);

print(pi(100));
