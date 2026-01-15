fn trim(s: &str) -> String {
    s.trim().to_string()
}

fn main() {
    println!("{}", trim(" hello ")); // hello
}
