fn trim_start(s: &str) -> String {
    s.trim_start().to_string()
}

fn main() {
    println!("{}", trim_start(" hello ")); // hello 
}
