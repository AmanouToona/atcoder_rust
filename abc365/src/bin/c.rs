use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
    }

    if A.iter().sum::<usize>() <= M {
        println!("infinite");
        return;
    }

    let mut ng = *A.iter().max().unwrap();
    let mut ok = 0;

    while ng - ok > 1 {
        let mid = (ng + ok) / 2;

        let sum = A.iter().map(|x| *x.min(&mid)).sum::<usize>();
        if sum <= M {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
