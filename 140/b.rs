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
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n-1];
    for i in 0..n {
        a[i] = read::<usize>() - 1;
    }
    for i in 0..n {
        b[i] = read::<usize>();
    }
    for i in 0..n-1 {
        c[i] = read::<usize>();
    }


    let mut ans = b[a[0]];
    let mut last = a[0];
    for i in 1..n {
        ans += b[a[i]];
        if last+1 == a[i] {
            ans += c[a[i-1]];
        }
        last = a[i];
    }

    println!("{}", ans)
}