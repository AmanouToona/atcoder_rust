use num_integer::Roots;
use proconio::input;

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = Vec::new();

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }

        let mut j = i + i;

        while j <= n {
            is_prime[j] = false;
            j += i;
        }
    }

    primes
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let primes = eratosthenes(N.sqrt());

    let mut ans = 0;
    let mut n = primes.len() - 1;

    for (i, &p) in primes.iter().enumerate() {
        while primes[n] * primes[n] * p * p > N && n > 0 && n > i {
            n -= 1;
        }

        if i >= n {
            break;
        }

        ans += n - i;
    }

    for &p in primes.iter() {
        if p.pow(8) > N {
            break;
        }
        ans += 1;
    }

    println!("{ans}");
}
