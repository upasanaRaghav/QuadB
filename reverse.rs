fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "hello";
    let output = reverse_string(input);
    println!("Reversed string: {}", output);
}
