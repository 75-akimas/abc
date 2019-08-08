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
    let h = read::<usize>();
    let w = read::<usize>();

    let mut a = vec![ vec!['.'; w]; h];
    let mut row = vec![0; h];
    let mut col = vec![0; w];
    for i in 0..h {
        a[i] = read::<String>().chars().collect();
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                row[i] = 1;
                col[j] = 1;
            }
        }
    }

    for i in 0..h {
        if row[i] == 0 {
            continue;
        }
        for j in 0..w {
            if col[j] == 0 {
                continue;
            }
            print!("{}", a[i][j]);
        }
        println!();
    }
}