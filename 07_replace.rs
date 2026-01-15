fn replace(s: &str, search: &str, rep: &str) -> String {
    s.replace(search, rep)
}

fn main() {
    println!("{}", replace("hello world", "world", "there")); // hello there
}
