use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [i64; 5],
    }

    let mut A2 = A.clone();
    A2.sort();

    let mut diff = 0;

    for (a, a2) in A.iter().zip(A2.iter()) {
        if diff == 1 {
            if a == a2 {
                println!("No");
                return;
            }
            diff += 1;
            continue;
        }

        if a != a2 {
            diff += 1;
            continue;
        }
    }

    if diff != 2 {
        println!("No");
    } else {
        println!("Yes");
    }
}
