use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for _ in 0..Q {
        input! {
            num: usize
        }

        if num == 1 {
            input! {x: usize}
            *map.entry(x).or_default() += 1;
        } else if num == 2 {
            input! {x: usize}
            *map.get_mut(&x).unwrap() -= 1;

            if map[&x] == 0 {
                map.remove(&x);
            }
        } else if num == 3 {
            println!("{}", map.len());
        } else {
            println!("bug");
        }
    }
}
