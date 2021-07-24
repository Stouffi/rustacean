fn fibonacci(n: u32) -> u32 {
  fibonacci_rec(n, 0, 1)
}

fn fibonacci_rec(n: u32, a: u32, b: u32) -> u32 {
  match n {
    0 => a,
    1 => b,
    n => fibonacci_rec(n - 1, b, a + b),
  }
}
