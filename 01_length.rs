fn length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let s = "hello";
    println!("{}", length(s)); // 5
}
