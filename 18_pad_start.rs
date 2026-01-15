fn pad_start(s: &str, target_len: usize, pad: char) -> String {
    let mut out = String::new();
    let current = s.chars().count();

    if current >= target_len {
        return s.to_string();
    }

    let need = target_len - current;
    for _ in 0..need {
        out.push(pad);
    }

    out.push_str(s);
    out
}

fn main() {
    println!("{}", pad_start("5", 3, '0')); // 005
}
