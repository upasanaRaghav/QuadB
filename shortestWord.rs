fn shortest_word(s: &str) -> String {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("").to_string()
}

fn main() {
    let input = "hello world this is Rust";
    let output = shortest_word(input);
    println!("Shortest word in '{}' is '{}'", input, output);
}
