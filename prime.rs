fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let input = 17;
    let output = is_prime(input);
    println!("Is {} prime? {}", input, output);
}
