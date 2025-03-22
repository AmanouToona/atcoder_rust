use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    for ((&a, &b), &c) in A.iter().zip(A.iter().skip(1)).zip(A.iter().skip(2)) {
        if a == b && b == c {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
