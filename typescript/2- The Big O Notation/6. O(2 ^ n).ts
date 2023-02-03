function fibonacci(n: number) {
  if (n <= 1) return n;

  // O(2 ^ n) The number of operations grows exponentially with the size of the input.
  return fibonacci(n - 1) + fibonacci(n - 2);
}
