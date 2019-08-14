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
    let n = read::<usize>();
    let mut ab = vec![(0, 0); n];

    for i in 0..n {
        ab[i].1 = read::<i64>();
        ab[i].0 = read::<i64>();
    }
    ab.sort_by(|a, b| a.cmp(b));

    let mut cg :Vec<usize> = Vec::new();
    cg.push(0);
    for i in 1..n {
        if ab[i].0 != ab[i-1].0 {
            cg.push(i);
        }
    }
    cg.push(ab.len());

    let mut time = 0;
    for i in 1..cg.len() {
        for j in (cg[i-1]..cg[i]).rev() {
            time += ab[j].1;
            if time > ab[j].0 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}