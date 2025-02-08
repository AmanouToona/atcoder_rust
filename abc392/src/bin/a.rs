use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [usize; 3],
    }

    if A[0] * A[1] == A[2] {
        println!("Yes");
        return;
    }
    if A[0] * A[2] == A[1] {
        println!("Yes");
        return;
    }
    if A[2] * A[1] == A[0] {
        println!("Yes");
        return;
    }
    println!("No");
}
