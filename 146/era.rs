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

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    // spf:
    // store smallest prime factor of number
    // 最小の素因数を格納
    // is_prime:
    // 配列番号が素数であるかどうかを示す
    let mut spf = vec![None; n+1];
    let mut is_prime = vec![true; n+1];
    let mut primes = Vec::new();

    // 0 と 1 は素数ではない
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n+1 {
        // is_prime[i]:
        // それ以下の素数で割り切られていなければ True となる
        if is_prime[i] {

            // この if 文より i は素数なので
            primes.push(i);

            // 素数はそれ自身の最も小さい素因数であるから
            spf[i] = Some(i);
        }

        // 全ての prime の倍数(prime * i)を取り除く
        for prime in &primes {
            // prime が i の最も小さい因数よりも大きくなってしまったら探索終了
            if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                break;
            }

            is_prime[i * prime] = false;

            // i * primes[j] の最も小さい素数は primes[j] 自身
            spf[i * prime] = Some(*prime);
        }
    }
    primes
}

// Prime Factorization using The Sieve of Eratosthenes
// エラストテネスの篩を用いた素因数分解
fn factorization_with_sieve_of_eratosthenes(mut n: usize) -> HashMap<usize, usize> {
    // primes:
    // ルートn までの素数
    // hm_primes:
    // ハッシュマップの素因数
    let mut primes = sieve_of_eratosthenes(f64::sqrt(n as f64) as usize);
    let mut hm_primes = HashMap::new();

    //  n を ルートn 以下の素数で割り切れるまで割っていく
    for prime in primes {
        while n % prime == 0 {
            n /= prime;
            if hm_primes.contains_key(&prime) {
                let mut x = hm_primes.get_mut(&prime).unwrap();
                *x += 1;
            } else {
                hm_primes.insert(prime, 1);
            }
        }
    }

    // 最後にnが素数になっている場合はそれ自身も素因数に含めて終了
    if n > 1 {
        if hm_primes.contains_key(&n) {
            let mut x = hm_primes.get_mut(&n).unwrap();
            *x += 1;
        } else {
            hm_primes.insert(n, 1);
        }
    }
    hm_primes
}

fn main() {
    let a = read::<usize>();
    let b = read::<usize>();


    println!("{:?}",factorization_with_sieve_of_eratosthenes(gcd(a, b)).len()+1);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}