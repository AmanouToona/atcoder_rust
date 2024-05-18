use proconio::input;

fn count0(right: i64, i: usize) -> usize {
    let mut res = right / 4;

    let mut n = res * 4;

    while n <= right {
        if n.rem_euclid(4) as usize == i {
            res += 1;
        }
        n += 1;
    }

    res as usize
}

fn count(left: i64, right: i64, i: usize) -> usize {
    let sub = if left < 0 {
        (-left / 4 + 1) * 4
    } else {
        (left / 4 + 1) * 4
    };

    count0(right + sub, i) - count0(left + sub, i)
}
#[allow(non_snake_case)]
fn main() {
    input! {(A, B): (i64, i64)}

    for i in 0..4 {
        let a = count(A, B, i);
        println!("{}, {}", i, a);
    }
}
