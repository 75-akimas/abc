use std::io::*;
use std::str::FromStr;

pub fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

use std::collections::HashMap;

fn main() {
    let N = read::<usize>();
    let mut h = HashMap::new();
    let mut old_w = String::new();
    let w = read::<String>();
    h.insert(w.to_string(), 1);
    old_w = w.to_string();

    for i in 1..N {
        let w = read::<String>();
        if !h.contains_key(&w) {
            h.insert(w.to_string(), 1);
        } else {
            break;
        }

        if old_w.chars().nth(old_w.len()-1).unwrap() != w.chars().nth(0).unwrap() {
            break;
        }

        if i == N-1 {
            println!("Yes");
            return;
        }

        old_w = w.to_string();
    }
    println!("No");
}