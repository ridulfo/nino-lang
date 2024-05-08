let factorial_helper:fn = (n:num, acc:num):num => n ? {
	0 => acc,
	factorial_helper(n - 1, n*acc)
};

let factorial:fn = (n:num):num => n ? {
        0 => 1,
	factorial_helper(n - 1, n)
};


let result1:num = factorial(0);
print(result1);
let result2:num = factorial(1);
print(result2);
let result3:num = factorial(5);
print(result3);
