fn ends_with(s: &str, sub: &str) -> bool {
    s.ends_with(sub)
}

fn main() {
    println!("{}", ends_with("hello world", "world")); // true
}
