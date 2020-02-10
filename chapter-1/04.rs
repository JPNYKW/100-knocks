use std::collections::HashMap;

fn main() {
    let s = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let ids = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];

    let mut map = HashMap::new();
    let mut i = 0;

    for t in s.split_whitespace() {
        i += 1;

        map.insert(
            t.chars().nth(if ids.contains(&i) { 0 } else { 1 }).unwrap(),
            i
        );
    }

    println!("{:#?}", map);
}

