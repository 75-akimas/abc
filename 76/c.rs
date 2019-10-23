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

fn main() {
    let s = read::<String>().chars().collect::<Vec<char>>();
    let t = read::<String>().chars().collect::<Vec<char>>();

    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }
    let mut v = Vec::new();
    for i in 0..(s.len() + 1 - t.len()) {
        for j in 0..t.len() {
            if s[i+j] != t[j] && '?' != s[i+j] {
                break;
            }
            if j == t.len()-1 {
                v.push(i);
            }
        }
    }
    if v.len() == 0 {
        println!("UNRESTORABLE");
        return;
    }

    for i in 0..s.len() {
        if s[i] == '?' {
            if v[v.len()-1] <= i && i < v[v.len()-1] + t.len() {
                print!("{}", t[i-v[v.len()-1]])
            } else {
                print!("a");
            }
        } else {
            print!("{}", s[i]);
        }
    }
}