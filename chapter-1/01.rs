fn main() {
    let s = "パタトクカシーー".to_string();
    let mut t = String::new();
    let mut i = 0;

    for c in s.as_str().chars() {
        i += 1;
        if i % 2 > 0 { t = format!("{}{}", t, c); }
    }

    println!("{}", t);
}

