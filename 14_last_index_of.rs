fn last_index_of(s: &str, sub: &str) -> i32 {
    match s.rfind(sub) {
        Some(pos) => pos as i32,
        None => -1,
    }
}

fn main() {
    println!("{}", last_index_of("hello world world", "world")); // 12
}
