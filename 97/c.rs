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
    let n = read::<usize>();
    let mut a :Vec<String> = vec![];
    for i in 1..6 {
        for j in 0..s.len() {
            let mut b = vec![];
            for k in 0..i {
                if j+k < s.len() {
                    b.push(s[k+j]);
                }
            }
            if !b.is_empty() && b.len() == i {
                let c :String = b.into_iter().collect();
                a.push(c);
            }
        }
    }
    a.sort();
    let mut cnt = 0;
    if a.len() == 1 {
        println!("{}", a[0]);
        return;
    }
    for i in 0..a.len()-1 {
        if a[i+1] != a[i] {
            cnt+=1;
        }
        if cnt == n-1 {
            println!("{}", a[i+1]);
            return;
        }
    }
}