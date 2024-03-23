use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {W: usize, B:usize};

    let key = "wbwbwwbwbwbw".to_string().repeat(100);
    let key: Vec<char> = key.chars().collect();

    for i in 0..12 {
        let mut w = 0;
        let mut b = 0;

        for &s in key.iter().skip(i) {
            if s == 'w' {
                w += 1;
            } else {
                b += 1;
            }

            if w == W && b == B {
                println!("Yes");
                return;
            }

            if w > W || b > B {
                break;
            }
        }
    }
    println!("No");
}
