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
    let n = read::<usize>() - 1;
    let mut csf = vec![(0, 0, 0); n];

    for i in 0..n {
        csf[i] = (read::<i64>(), read::<i64>(), read::<i64>());
    }

    for i in 0..n {
        let mut a = csf[i].0 + csf[i].1;
        for j in i+1..n {
            if a - csf[j].1 > 0 {
                let b = (a - csf[j].1) % csf[j].2;
                a += csf[j].0;
                if b != 0 {
                    a += csf[j].2 - b;
                }
            } else {
                a = csf[j].0 + csf[j].1;
            }
        }
        println!("{}", a);
    }
    println!("0")
}