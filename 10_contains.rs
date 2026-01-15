fn contains(s: &str, sub: &str) -> bool {
    s.contains(sub)
}

fn main() {
    println!("{}", contains("hello", "ell")); // true
}
