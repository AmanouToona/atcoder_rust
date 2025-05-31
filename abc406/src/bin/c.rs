use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    let mut ans = 0;
    let mut top = 0;
    let mut bottom = 0;
    let mut right = 1;
    for left in 1..N - 1 {
        if P[left - 1] < P[left] && P[left] > P[left + 1] {
            top -= 1;
        } else if P[left - 1] > P[left] && P[left] < P[left + 1] {
            bottom -= 1;
        }

        while right + 1 < N {
            right += 1;

            if P[right - 2] < P[right - 1] && P[right - 1] > P[right] {
                top += 1;
            } else if P[right - 2] > P[right - 1] && P[right - 1] < P[right] {
                bottom -= 1;
            }

            if top == 1 && right == 1 && P[left - 1] < P[left] {
                ans += 1;
            }

            if bottom > 1 || top > 1 {
                break;
            }
        }

        if right == left {
            if P[right - 2] < P[right - 1] && P[right - 1] > P[right] {
                top += 1;
            } else if P[right - 2] > P[right - 1] && P[right - 1] < P[right] {
                bottom -= 1;
            }
            right += 1;
        }
    }
    println!("{ans}");
}
