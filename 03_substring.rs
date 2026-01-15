fn substring(s: &str, start: usize, len: usize) -> String {
    let mut out = String::new();

    for (i, ch) in s.chars().enumerate() {
        if i >= start && i < start + len {
            out.push(ch);
        }
    }

    out
}

fn main() {
    let s = "HelloWorld";
    println!("{}", substring(s, 3, 5)); // loWor
}
