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
    let mut xyh = vec![(0, 0, 0); n];

    for i in 0..n {
        xyh[i].0 = read::<i64>();
        xyh[i].1 = read::<i64>();
        xyh[i].2 = read::<i64>();
    }

    for i in 0..101 {
        for j in 0..101 {
            let mut needH = -1;

            for k in 0..n {
                if xyh[k].2 > 0 {
                    let tmp = xyh[k].2 + (i - xyh[k].1).abs() + (j - xyh[k].0).abs();
                    if needH == -1 {
                        needH = tmp;
                    } else {
                        if needH != tmp {
                            needH = -2;
                            break;
                        }
                    }
                }
            }
            if needH == -2 {
                continue;
            }

            for k in 0..n {
                if xyh[k].2 == 0 {
                    let dist = (i - xyh[k].1).abs() + (j - xyh[k].0).abs();
                    if needH > dist {
                        needH = -2;
                    }
                }
            }

            if needH == -2 {
                continue;
            }

            println!("{} {} {}", j, i, needH);
        }
    }
}