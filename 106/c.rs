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
    let s_orig = read::<String>();
    let k = read::<i64>();
    let s = s_orig.chars().collect::<Vec<char>>();
    let mut ib :i64 = 0;
    for i in 0..s_orig.len() {
        ib = i as i64;
        if s[i] != '1' {
            break;
        }
    }

    if k > ib {
        println!("{}", s[ib as usize]);
    } else {
        println!("1");
    }
}