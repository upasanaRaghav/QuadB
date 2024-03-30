fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let input = "upasana";
    let output = is_palindrome(input);
    println!("Is '{}' a palindrome? {}", input, output);
}



