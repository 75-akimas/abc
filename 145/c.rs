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

    let mut x:Vec<f64> = vec![0.0; n];
    let mut y:Vec<f64> = vec![0.0; n];
    let mut z = vec![0;n];
    for i in 0..n {
        x[i] = read::<f64>();
        y[i] = read::<f64>();
        z[i] = i+1;
    }

    let mut cnt:f64 = 0.0;
    let mut sum:f64 = 0.0;

    for a in 0..n {
        for b in 0..n {
            if a == b {
                continue;
            }
            if n == 2 {
                cnt +=1.0;
                sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt();
            }
            for c in 0..n {
                if b == c || a == c {
                    continue;
                }
                if n == 3 {
                    cnt +=1.0;
                    sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt();
                }
                if n < 3 {
                    break;
                }

                for d in 0..n {
                    if a == d || b == d || c == d {
                        continue;
                    }
                    if n == 4 {
                        cnt +=1.0;
                        sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt() + ((x[c] - x[d])*(x[c] - x[d]) + (y[c] - y[d])*(y[c] - y[d])).sqrt();
                    }
                    if n < 4 {
                        break;
                    }
                    for e in 0..n {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }
                        if n == 5 {
                            cnt +=1.0;
                            sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt() + ((x[c] - x[d])*(x[c] - x[d]) + (y[c] - y[d])*(y[c] - y[d])).sqrt() + ((x[d] - x[e])*(x[d] - x[e]) + (y[d] - y[e])*(y[d] - y[e])).sqrt();
                        }
                        if n < 5 {
                            break;
                        }
                        for f in 0..n {
                            if a == f || b == f || c == f || d == f || e == f {
                                continue;
                            }
                            if n == 6 {
                                cnt +=1.0;
                                sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt() + ((x[c] - x[d])*(x[c] - x[d]) + (y[c] - y[d])*(y[c] - y[d])).sqrt() + ((x[d] - x[e])*(x[d] - x[e]) + (y[d] - y[e])*(y[d] - y[e])).sqrt() + ((x[e] - x[f])*(x[e] - x[f]) + (y[e] - y[f])*(y[e] - y[f])).sqrt();
                            }
                            if n < 6 {
                                break;
                            }
                            for g in 0..n {
                                if a == g || b == g || c == g || d == g || e == g || f == g {
                                    continue;
                                }
                                if n == 7 {
                                    cnt +=1.0;
                                    sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt() + ((x[c] - x[d])*(x[c] - x[d]) + (y[c] - y[d])*(y[c] - y[d])).sqrt() + ((x[d] - x[e])*(x[d] - x[e]) + (y[d] - y[e])*(y[d] - y[e])).sqrt() + ((x[e] - x[f])*(x[e] - x[f]) + (y[e] - y[f])*(y[e] - y[f])).sqrt() + ((x[f] - x[g])*(x[f] - x[g]) + (y[f] - y[g])*(y[f] - y[g])).sqrt();
                                }
                                if n < 7 {
                                    break;
                                }
                                for h in 0..n {
                                    if a == h || b == h || c == h || d == h || e == h || f == h || g == h {
                                        continue;
                                    }
                                    if n == 8 {
                                        cnt +=1.0;
                                        sum += ((x[a] - x[b])*(x[a] - x[b]) + (y[a] - y[b])*(y[a] - y[b])).sqrt() + ((x[b] - x[c])*(x[b] - x[c]) + (y[b] - y[c])*(y[b] - y[c])).sqrt() + ((x[c] - x[d])*(x[c] - x[d]) + (y[c] - y[d])*(y[c] - y[d])).sqrt() + ((x[d] - x[e])*(x[d] - x[e]) + (y[d] - y[e])*(y[d] - y[e])).sqrt() + ((x[e] - x[f])*(x[e] - x[f]) + (y[e] - y[f])*(y[e] - y[f])).sqrt() + ((x[f] - x[g])*(x[f] - x[g]) + (y[f] - y[g])*(y[f] - y[g])).sqrt() + ((x[g] - x[h])*(x[g] - x[h]) + (y[g] - y[h])*(y[g] - y[h])).sqrt();
                                    }
                                    if n < 8 {
                                        break;
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum/cnt);
}