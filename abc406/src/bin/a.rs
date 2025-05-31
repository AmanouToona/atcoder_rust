use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C,D) : (usize, usize, usize, usize)
    }

    if A > C {
        println!("Yes");
        return;
    } else if A < C {
        println!("No");
        return;
    } else {
        if B < D {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
