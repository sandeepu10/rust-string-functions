fn index_of(s: &str, sub: &str) -> i32 {
    match s.find(sub) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

fn main() {
    println!("{}", index_of("hello world", "world")); // 6
}
