use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, S): (usize, usize),
        A: [usize; N],
    }

    let sumA: usize = A.iter().sum();
    let A2: Vec<usize> = A.iter().cycle().cloned().take(2 * N).collect();
    let mut cumsum = vec![0; 2 * N + 1];

    for (i, &a2) in A2.iter().enumerate() {
        cumsum[i + 1] = a2;
        cumsum[i + 1] += cumsum[i];
    }

    let S = S % sumA;

    for l in 0..N {
        let mut r = 2 * N;

        if cumsum[r] - cumsum[l] == S {
            println!("Yes");
            return;
        }

        let mut l2 = l;
        while r - l2 > 1 {
            let mid = (r + l2) / 2;

            if cumsum[mid] - cumsum[l] > S {
                r = mid;
            } else {
                l2 = mid;
            }
        }

        if cumsum[l2] - cumsum[l] == S {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
