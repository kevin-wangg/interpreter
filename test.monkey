let x = 5;
let y = 10;
let result = x + y;

# This is a comment
def factorial(n) {
	if (n == 1) {
		return 1;
	}
	return n * factorial(n - 1);
}

def fib(n) {
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
}

# Should return 797681364480000
factorial(result) * fib(result)
