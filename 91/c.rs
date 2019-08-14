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
    let mut cd = vec![(0, 0); n];
    for i in 0..n {
        ab[i] = (read::<usize>(), read::<usize>());
    }
    for i in 0..n {
        cd[i] = (read::<usize>(), read::<usize>());
    }

    ab.sort();
    cd.sort_by_key(|k| k.1);

    let mut ans = vec![vec![]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            if ab[j].0 < cd[i].0 && ab[j].1 < cd[i].1 {
                if ans[j] == vec![] {
                    ans[j].push(i);
                    break;
                }
            }
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        if ans[i] != vec![] {
            cnt+=1;
        }
    }
    println!("{:?}", cnt);
}