use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {W: usize, B:usize};

    let key = "wbwbwwbwbwbw";

    'outer: for i in 0..12 {
        let mut w = 0;
        let mut b = 0;

        for s in key.chars().cycle().skip(i) {
            match s {
                'w' => w += 1,
                'b' => b += 1,
                _ => {}
            };

            if w == W && b == B {
                println!("Yes");
                return;
            }

            if w > W || b > B {
                continue 'outer;
            }
        }
    }

    println!("No");
}
