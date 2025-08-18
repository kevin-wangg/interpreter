let x = 5;
let y = 10;
let result = x + y;

# This is a comment
let rec factorial = fun(n) {
	if (n == 1) {
		return 1;
	}
	return n * factorial(n - 1);
};

let rec fib = fun(n) {
	# This is another comment
	if (n == 1) {
		1
	} else {
		if (n == 2) {
			1
		} else {
			fib(n - 1) + fib(n - 2)
		} 
	}
};

println(123)

# Should return 797681364480000
println(factorial(result) * fib(result))

let rec map_helper = fun(ret, arr, f, index) {
	if index < len(arr) {
		let item = f(arr[index]);
		map_helper(push(ret, item), arr, f, index + 1)
	} else {
		ret
	}
};

let map = fun(arr, f) {
	map_helper([], arr, f, 0)
};

let ret = map([1,2,100], fun(n) { n * n });

println(ret)
