use proconio::input;
use proconio::marker::Chars;

struct Node {
    key: char,
    cnt: usize,
    child: Vec<Node>,
}


struct Solve {
    tree: Vec<Node>,
}

impl Solve {
    fn new() -> Self{
        Solve{
            tree: vec![],
        }
    }

    fn add(&mut self, s: &Chars) {
        for n in self.tree.iter() {
            if n.key == s[0] {
                &n.key += 1
            }
        }
    }
}


#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let mut tree = Vec::new();

    for S


    let mut ans = 0;
}
