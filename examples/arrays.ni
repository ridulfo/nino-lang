let arr1: [num] = [1, 2, 3];
let arr2: [num] = [4, 5, 6];
print(arr1 + arr2);
print(arr1 == arr2);

let cat:fn = (a:[char], b:[char]):[char] => a + b;
print(cat("Hello", " world"));

# Needed because the parser cannot currently infer that [head("abc")] has the evaluated type [char]
let make_char_array:fn = (c:char):[char] => tail(['a', c]);

let reverse:fn = (a: [char]):[char] => (len(a) > 1) ? {
    true => reverse(tail(a)) + make_char_array(head(a)),
    false => a
};

print(reverse("Hello world!"));
