use proconio::input;
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut primes = Vec::new();
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        primes.push(i);

        for j in 1..=(n / i) {
            is_prime[i * j] = false;
        }
    }

    primes
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N : usize,
        A: [usize; N],
    }

    // nim, 約数の数の xor
    let max = 10usize.pow(5u32);

    let primes = eratosthenes(max);

    let mut num_divsors = vec![0; max + 1];

    for &p in primes.iter() {
        let mut pp: usize = p;
        while pp < max {
            for i in 1..=(max / pp) {
                num_divsors[pp * i] += 1;
            }
            pp *= p;
        }
    }

    let mut ans = 0;
    for &a in A.iter() {
        ans ^= num_divsors[a];
    }

    if ans == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
    }
}
