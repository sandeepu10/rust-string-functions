fn split(s: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    for x in s.split(delimiter) {
        parts.push(x.to_string());
    }
    parts
}

fn main() {
    let result = split("a,b,c", ',');
    println!("{:?}", result); // ["a", "b", "c"]
}
