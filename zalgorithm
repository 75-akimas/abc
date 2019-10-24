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
    let s = read::<String>().chars().collect::<Vec<char>>();

    let mut a = Vec::new();
    let mut c = Vec::new();
    for i in 0..n {
        a.push(vec![0; n-i]);
        c.push(Vec::new());
        for j in i..n {
            c[i].push(s[j]);
        }
    }

    for b in 0..n {
        a[b][0] = n-b;
        let mut i = 1;
        let mut j = 0;
        while i < n-b {
            while i+j < n-b && c[b][j] == c[b][i+j] { j+=1 }
            a[b][i] = j;
            if j == 0 { i+=1; continue; }
            let mut k = 1;
            while i+k < n-b && k+a[b][k] < j { a[b][i+k] = a[b][k]; k+=1; }
            i+=k;
            j-=k;
        }
    }

    let mut ans = 0;
    for i in 0..n-1 {
        for j in 0..n-i {
            if a[i][j] > ans {
                ans = std::cmp::max(ans, std::cmp::min(j, a[i][j]));
            }
        }
    }
    println!("{}", ans);
}