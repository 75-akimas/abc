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
    let s = read::<String>().chars().collect::<Vec<char>>();
    let t = read::<String>().chars().collect::<Vec<char>>();

    let mut h1 = HashMap::new();
    let mut h2 = HashMap::new();
    for i in 0..s.len() {
        let aa = h1.entry(s[i]).or_insert(t[i]);
        let bb = h2.entry(t[i]).or_insert(s[i]);
        if *aa != t[i] || *bb != s[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}