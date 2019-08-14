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
    let n = read::<usize>();
    let mut h = HashMap::new();
    h.insert('M', 0);
    h.insert('A', 1);
    h.insert('R', 2);
    h.insert('C', 3);
    h.insert('H', 4);
    let mut s = vec![vec![]; n];
    let mut ss = vec![0; 5];
    for i in 0..n {
        s[i] = read::<String>().chars().collect::<Vec<char>>();
        if h.get(&s[i][0]) != None {
            ss[h[&s[i][0]]] += 1;
        }
    }
    let mut ans:i64 = 0;
    for i in 0..3 {
        for j in i+1..4 {
            for k in j+1..5 {
                ans += ss[i] * ss[j] * ss[k];
            }
        }
    }
    println!("{}", ans);
}
