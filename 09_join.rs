fn join(arr: Vec<&str>, sep: &str) -> String {
    arr.join(sep)
}

fn main() {
    let arr = vec!["a", "b", "c"];
    println!("{}", join(arr, "-")); // a-b-c
}
