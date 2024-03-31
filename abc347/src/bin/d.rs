use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b, C): (i128, i128, i128),
    }

    let c = C.count_ones() as i128;

    // C 0 のところで、 A, B が 1になる個数
    let dup = (a + b - c) / 2;

    let mut A = 0;
    // A の C 0, A 1 を決定する
    let mut not_C = !C;
    for _ in 0..dup {
        let to_one = not_C & (-not_C);
        A |= to_one;
        not_C -= to_one;
    }

    // A の C 1, A 1 を決定する
    let mut C_ = C;
    let cnt = a - A.count_ones() as i128;
    for _ in 0..cnt {
        let to_one = C_ & (-C_);
        A |= to_one;
        C_ -= to_one;
    }

    let B = C ^ A;

    if A >= 2i128.pow(60)
        || B >= 2i128.pow(60)
        || A.count_ones() as i128 != a
        || B.count_ones() as i128 != b
        || A ^ B != C
    {
        println!("-1");
        return;
    }

    println!("{} {}", A, B);
}
