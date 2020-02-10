fn main() {
    let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let mut l = Vec::new();

    for t in s.split_whitespace() { l.push(t.len()); }
    println!("{:?}", l);
}

