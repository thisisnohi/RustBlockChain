#[test]
fn main() {
    let mut hello = "你好".to_string();
    hello += " world";
    println!("{}", hello);

    let s = format!("{}, {}", "你好", " worlllld");
    println!("{}", s);
}
