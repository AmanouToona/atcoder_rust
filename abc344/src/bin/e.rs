use im_rc::HashMap;
use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        n: usize,
        A: [usize; n],
        Q: usize,
    }

    let mut parent: HashMap<usize, usize> = HashMap::new();
    let mut child: HashMap<usize, usize> = HashMap::new();

    parent.insert(A[0], 0);
    child.insert(0, A[0]);
    child.insert(A[n - 1], usize::MAX);

    for (&a1, &a2) in A.iter().zip(A[1..].iter()) {
        parent.insert(a2, a1);
        child.insert(a1, a2);
    }

    for _ in 0..Q {
        input! {n: usize};
        if n == 1 {
            input! {
                (x, y): (usize, usize),
            }

            child.insert(y, child[&x]);
            parent.insert(child[&x], y);
            parent.insert(y, x);
            child.insert(x, y);
        } else {
            input! {
                x: usize,
            }

            let p = parent[&x];
            let c = child[&x];

            child.insert(p, c);
            parent.insert(c, p);
        }
    }

    let mut ans = vec![0];

    loop {
        let p = ans.last().unwrap();

        if child[p] == usize::MAX {
            break;
        }
        ans.push(child[p]);
    }

    let ans = ans.iter().skip(1).map(|x| x.to_string()).join(" ");
    println!("{}", ans);
}
