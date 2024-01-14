let arr1: [num] = [1, 2, 3];
let arr2: [num] = [4, 5, 6];
print(arr1 + arr2);
print(arr1 == arr2);

let cat:fn = (a:[char], b:[char]):[char] => a + b;
print(cat("Hello", " world"));
