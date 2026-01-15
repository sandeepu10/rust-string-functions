fn starts_with(s: &str, sub: &str) -> bool {
    s.starts_with(sub)
}

fn main() {
    println!("{}", starts_with("hello world", "hello")); // true
}
