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
    let mut c = vec![vec![0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            c[i][j] = read::<i32>();
        }
    }

    let mut a = vec![0; 3];
    let mut b = vec![0; 3];
    a[1] = c[0][1] - c[0][0];
    a[2] = c[0][2] - c[0][0];
    b[1] = c[1][0] - c[0][0];
    b[2] = c[2][0] - c[0][0];
    for i in 1..3 {
        if !(c[i][1] - c[i][0] == a[1] && c[i][2] - c[i][0] == a[2]) {
            println!("No");
            return;
        }
    }
    for i in 1..3 {
        if !(c[1][i] - c[0][i] == b[1] && c[2][i] - c[0][i] == b[2]) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}