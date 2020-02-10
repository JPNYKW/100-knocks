fn main() {
    let s = "パトカー".to_string();
    let t = "タクシー".to_string();
    let mut u = String::new();

    for i in 0..s.chars().count() {
        u = format!("{}{}{}",
            u,
            s.chars().nth(i).unwrap(),
            t.chars().nth(i).unwrap()
        );
    }

    println!("{}", u);
}

