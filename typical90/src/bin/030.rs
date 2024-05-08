use proconio::input;

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut prime = Vec::new();
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }

        prime.push(i);

        for j in ((i * 2)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }

    prime
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
    }

    let primes = eratosthenes(N + 1);

    let mut cnt = vec![0; N + 1];

    for &p in primes.iter() {
        for i in (p..=N).step_by(p) {
            cnt[i] += 1;
        }
    }

    let mut ans = 0;
    for c in cnt {
        if c >= K {
            ans += 1
        }
    }

    println!("{}", ans);
}
