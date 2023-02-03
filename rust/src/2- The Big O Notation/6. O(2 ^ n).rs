fn fibonacci(n: i32) {
    if n <= 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2);
}
