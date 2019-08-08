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

use std::cmp;

fn main() {
    let nmxy = (read::<usize>(), read::<usize>(), read::<i32>(), read::<i32>());
    let mut x = vec![0; nmxy.0];
    let mut y = vec![0; nmxy.1];

    for i in 0..nmxy.0 {
        x[i] = read::<i32>();
    }
    for i in 0..nmxy.1 {
        y[i] = read::<i32>();
    }


    let mut x_max = *x.iter().max().unwrap();
    let mut y_min = *y.iter().min().unwrap();
    x_max = cmp::max(x_max, nmxy.2);
    y_min = cmp::min(y_min, nmxy.3);

    if x_max >= y_min {
        println!("War");
    } else {
        println!("No War");
    }
}