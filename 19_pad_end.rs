fn pad_end(s: &str, target_len: usize, pad: char) -> String {
    let mut out = String::new();
    let current = s.chars().count();

    out.push_str(s);

    if current >= target_len {
        return out;
    }

    let need = target_len - current;
    for _ in 0..need {
        out.push(pad);
    }

    out
}

fn main() {
    println!("{}", pad_end("5", 3, '0')); // 500
}
