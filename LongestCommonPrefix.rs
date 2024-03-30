fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = strings[0].clone();
    for (i, c) in first.chars().enumerate() {
        if !strings.iter().all(|s| s.chars().nth(i) == Some(c)) {
            return first[..i].to_string();
        }
    }
    first
}

fn main() {
    let strings = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let output = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", output);
}
