use im_rc::HashMap;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {N: usize,

            A:[usize; N],
            B:[usize; N],
            C:[usize; N],
    };

    let mut Ac: HashMap<usize, usize> = HashMap::new();
    let mut Bc: HashMap<usize, usize> = HashMap::new();
    let mut Cc: HashMap<usize, usize> = HashMap::new();

    for a in A.iter() {
        *Ac.entry(*a % 46).or_default() += 1;
    }

    for b in B.iter() {
        *Bc.entry(*b % 46).or_default() += 1;
    }
    for c in C.iter() {
        *Cc.entry(*c % 46).or_default() += 1;
    }

    let mut ans = 0;

    for (a, ac) in Ac.iter() {
        for (b, bc) in Bc.iter() {
            for (c, cc) in Cc.iter() {
                if (a + b + c) % 46 == 0 {
                    ans += ac * bc * cc;
                }
            }
        }
    }

    println!("{}", ans);
}
