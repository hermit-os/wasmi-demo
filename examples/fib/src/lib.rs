// Calculating fibonacci numbers
#[no_mangle]
pub extern "C" fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[no_mangle]
pub extern "C" fn bench(iterations: usize, number: u64) {
    for _ in 0..iterations {
        fibonacci(number);
    }
}