fn concat(a: &str, b: &str) -> String {
    let mut res = String::new();
    res.push_str(a);
    res.push_str(b);
    res
}

fn main() {
    println!("{}", concat("Hello", "World")); // HelloWorld
}
