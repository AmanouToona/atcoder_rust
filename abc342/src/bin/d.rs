use im_rc::HashMap;
use proconio::input;
// sieve of Eratosthenes
fn eratosthenes<T>(n: T) -> Vec<T>
where
    T: Copy
        + TryInto<usize>
        + TryFrom<usize>
        + std::ops::Rem<Output = T>
        + std::cmp::PartialOrd
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::marker::Sized,
{
    let n_usize: usize = match n.try_into() {
        Ok(num) => num,
        Err(_) => return Vec::new(),
    };

    let mut is_prime = vec![true; n_usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primse: Vec<T> = Vec::new();

    for i in 2..n_usize {
        if !is_prime[i] {
            continue;
        }

        match T::try_from(i) {
            Ok(val) => {
                primse.push(val);
                let mut v = i;
                while v + i < n_usize {
                    v += i;
                    is_prime[i] = false;
                }
            }
            Err(_) => break,
        }
    }
    primse
}

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut A: [usize; n],
    }

    let primes = eratosthenes(n);

    for a in A.iter_mut() {
        for p in primes.iter() {
            if p * p > *a {
                break;
            }

            while *a % (p * p) == 0 {
                *a /= p * p;
            }
        }
    }

    let mut cnt = HashMap::new();
    for a in A.iter() {
        *cnt.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (&k, &v) in cnt.iter() {
        if k == &0 {
            ans += (2 * n - v - 1) * v / 2;
        } else {
            ans += v * (v - 1) / 2;
        }
    }

    println!("{:?}", ans);
}
