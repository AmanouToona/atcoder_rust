use proconio::input;
fn main() {
    input! {
        t: String,
        n : usize,
        a_s: [String; n],
    }

    let mut a = Vec::new();

    for _a in a_s.into_iter() {
        let _a: Vec<String> = _a
            .split(' ')
            .enumerate()
            .filter(|(i, _)| i != &0)
            .map(|(_, s)| s.to_string())
            .collect();
        a.push(_a);
    }

    let t: Vec<char> = t.chars().collect();

    fn dfs(cost: usize, i: usize, s: usize, a: &Vec<Vec<String>>, t: &Vec<char>) -> usize {
        let mut in_cost = 10usize.pow(10);

        if i == a.len() {
            if s == t.len() {
                return cost;
            }
            return in_cost;
        }

        in_cost = in_cost.min(dfs(cost, i + 1, s, a, t));

        for a_ in &a[i] {
            if s + a_.len() > t.len() {
                continue;
            }

            let mut can = true;

            for (a__, &t_) in a_.chars().zip(t[s..s + a_.len()].iter()) {
                if a__ != t_ {
                    can = false;
                    break;
                }
            }

            if can {
                in_cost = in_cost.min(dfs(cost + 1, i + 1, s + a_.len(), a, t));
            }
        }

        return in_cost;
    };

    let ans = dfs(0, 0, 0, &a, &t);

    if ans < 10000000000 {
        println!("{}", ans);
    } else {
        println!("-1")
    }
}
