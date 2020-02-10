fn main() {
    let s = "パタトクカシーー".to_string();
    let mut t = String::new();

    for i in 0..s.chars().count() { if i % 2 > 0 { t = format!("{}{}", t, s.chars().nth(i).unwrap()); } }
    println!("{}", t);
}

