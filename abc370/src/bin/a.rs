use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (L, R): (usize, usize),
    }

    if L + R == 0 || L + R == 2 {
        println!("Invalid");
    } else if L == 1 {
        println!("Yes");
    } else if R == 1 {
        println!("No");
    } else {
        println!("oop: {L}, {R}");
    }
}
