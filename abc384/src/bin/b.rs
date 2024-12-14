use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, R) : (usize, i64),
    }

    let mut R = R;

    for _ in 0..N {
        input! {
            (D, A): (usize, i64)
        }

        match D {
            1 => {
                if R >= 1600 && R < 2800 {
                    R += A;
                }
            }
            2 => {
                if R >= 1200 && R <= 2399 {
                    R += A;
                }
            }
            _ => {
                println!("ooop!!!");
            }
        }
    }

    println!("{R}");
}
