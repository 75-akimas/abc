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
    let mut x = read::<i64>();
    let mut a = vec![0; 4];
    for i in 0..4 {
        a[3-i] = x % 10;
        x /= 10;
    }

    let mut c = vec!['+'; 3];
    for o in 0..2_i32.pow(3) {
        let mut tmp = a[0];
        for i in 0..3 {
            if 1 << i & o != 0 {
                tmp += a[i+1];
                c[i] = '+';
            } else {
                tmp -= a[i+1];
                c[i] = '-';
            }
        }
        if tmp == 7 {
            break;
        }
    }
    print!("{}", a[0]);
    for i in 0..3 {
        print!("{}{}", c[i], a[i+1])
    }
    println!("=7")
}