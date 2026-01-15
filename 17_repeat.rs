fn repeat(s: &str, count: usize) -> String {
    let mut out = String::new();
    for _ in 0..count {
        out.push_str(s);
    }
    out
}

fn main() {
    println!("{}", repeat("ab", 3)); // ababab
}
