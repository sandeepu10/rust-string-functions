fn char_at(s: &str, index: usize) -> char {
    s.chars().nth(index).unwrap_or('\0')
}

fn main() {
    println!("{}", char_at("hello", 1)); // e
}
