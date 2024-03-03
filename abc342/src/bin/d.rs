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

    for i in 2..=n_usize {
        if !is_prime[i] {
            continue;
        }

        match T::try_from(i) {
            Ok(val) => primse.push(val),
            Err(_) => break,
        }
    }

    primse
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let primes = eratosthenes(n);


}
