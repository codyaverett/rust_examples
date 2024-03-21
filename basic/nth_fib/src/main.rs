
// Function to generate the nth Fibonacci number
fn nth_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    let mut n = String::new();
    println!("Enter the value of n: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    let result = nth_fib(n);
    println!("The {}th Fibonacci number is {}", n, result);
}
