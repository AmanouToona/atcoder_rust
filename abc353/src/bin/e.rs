use im_rc::HashMap;
use proconio::input;

#[derive(Clone)]
struct Node {
    child: HashMap<char, Node>,
    cnt: usize,
}

impl Node {
    fn new() -> Self {
        Node {
            child: HashMap::new(),
            cnt: 0,
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    };

    let mut node = Node::new();
    let mut ans = 0;

    for s in S.iter() {
        let mut node = &mut node;
        for c in s.chars() {
            node = node.child.entry(c).or_insert(Node::new());
            ans += node.cnt;
            node.cnt += 1;
        }
    }

    println!("{}", ans);
}
