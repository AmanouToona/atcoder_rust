use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (sx, sy): (i64, i64),
        (tx, ty): (i64, i64),
    }

    let sx = if (sx + sy) % 2 != 0 { sx - 1 } else { sx };
    let tx = if (tx + ty) % 2 != 0 { tx - 1 } else { tx };

    let h = (sy - ty).abs();
    let w = (sx - tx).abs();

    let mut ans = 0;
    ans += h;

    if w > h {
        ans += (w - h + 1) / 2;
    }

    println!("{}", ans);
}
