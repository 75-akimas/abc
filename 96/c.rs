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

    let mut s = vec![vec!['a'; w]; h];

    for i in 0..h {
        s[i] = read::<String>().chars().collect::<Vec<char>>();
    }
    let range :Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                let mut flag = 0;
                for k in 0..4 {
                    if 0 <= i as i32+range[k].0 && i as i32+range[k].0 < h as i32 && 0 <= j as i32+range[k].1 && j as i32+range[k].1 < w as i32 {
                        if s[(i as i32+range[k].0) as usize][(j as i32+range[k].1) as usize] == '#' {
                            flag = 1;
                        }
                    }
                }
                if flag == 0 {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}