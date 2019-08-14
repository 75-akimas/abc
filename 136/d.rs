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
    let s = read::<String>().chars().collect::<Vec<char>>();

    let n = s.len();
    let mut ans = vec![0; n];
    let mut i = 0;
    while i < n {
        let mut mv = i;
        let mut cnt = 0;
        let past = s[mv];

        loop {
            if past != s[mv] {
                if past == 'R' {
                    cnt = i + cnt - (cnt % 2);
                } else {
                    cnt = i - cnt + (cnt % 2);
                }
                ans[cnt] += 1;

                while i+1 < n && s[i] == s[i+1] {
                    i+=1;
                    if s[i] == 'R' {
                        if s[cnt] == 'R' {
                            cnt+=1;
                        } else {
                            cnt-=1;
                        }
                    } else {
                        if s[cnt] == 'R' {
                            cnt+=1;
                        } else {
                            cnt-=1;
                        }
                    }
                    ans[cnt] += 1;
                }
                break;
            }

            if s[mv] == 'R' {
                mv += 1;
            } else {
                mv -= 1;
            }
            cnt+=1;
        }
        i += 1;
    }
    for i in 0..n {
        print!("{} ", ans[i])
    }
}