fn char_code_at(s: &str, index: usize) -> i32 {
    match s.chars().nth(index) {
        Some(c) => c as i32,
        None => -1,
    }
}

fn main() {
    println!("{}", char_code_at("hello", 0)); // 104
}
