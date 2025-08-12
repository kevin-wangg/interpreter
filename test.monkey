let x = 5;
let y = 10;
let result = x + y;

def factorial(n) {
	if (n == 1) {
		return 1;
	} else {
		return n * factorial(n - 1);
	}
}

def fib(n) {
	if (n == 1) {
		return 1;
	} 
	if (n == 2) {
		return 1;
	}
	return fib(n - 1) + fib(n - 2);
}

factorial(result)
fib(result)
