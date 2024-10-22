use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]

struct State {
    R: Vec<usize>,
    ans: Vec<Vec<usize>>,
    K: usize,
}

#[allow(non_snake_case)]
impl State {
    fn new(R: Vec<usize>, K: usize) -> Self {
        State {
            R,
            ans: Vec::new(),
            K,
        }
    }
}

fn dfs(state: &mut State, now: &mut Vec<usize>) {
    if now.len() == state.R.len() {
        if now.iter().sum::<usize>() % state.K == 0 {
            state.ans.push(now.clone());
        }
        return;
    }

    for i in 1..=(state.R[now.len()]) {
        now.push(i);
        dfs(state, now);
        now.pop();
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        R: [usize; N],
    }

    let mut state = State::new(R, K);
    let mut now = Vec::new();
    dfs(&mut state, &mut now);

    let ans = state.ans.clone();
    for a in ans.iter() {
        let a: String = a.iter().join(" ");
        println!("{a}");
    }
}
