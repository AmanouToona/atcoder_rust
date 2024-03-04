use ac_library::MfGraph;
use proconio::input;
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [String; n],
    }

    let mut ss: Vec<Vec<char>> = Vec::new();

    for s_ in s.into_iter() {
        ss.push(s_.chars().into_iter().collect());
    }

    let ss = ss;

    let mut mf = MfGraph::<usize>::new(n * m + 2);
    let start = n * m;
    let goal = n * m + 1;
    let get_index = |r: usize, c: usize| r * m + c;

    // 左上が黒に市松模様に塗り分けて、黒から白の2部マッチングを行う
    for (r, s) in ss.iter().enumerate() {
        for (c, &s_) in s.iter().enumerate() {
            if s_ == '#' {
                continue;
            }

            if (r + c) % 2 == 0 {
                for (dr, dc) in [0, !0, 0, 1].iter().map(|i| *i as usize).zip([1, 0, !0, 0]) {
                    let vr = r.wrapping_add(dr);
                    let vc = c.wrapping_add(dc);

                    if (vr >= n) | (vc >= m) {
                        continue;
                    }

                    if ss[vr][vc] == '#' {
                        continue;
                    }

                    mf.add_edge(get_index(r, c), get_index(vr, vc), 1);
                }
            }

            if (r + c) % 2 == 0 {
                mf.add_edge(start, get_index(r, c), 1);
            } else {
                mf.add_edge(get_index(r, c), goal, 1);
            }
        }
    }
    mf.flow(start, goal);

    let edges = mf.edges();
    let mut cnt = 0;

    let mut ans = ss;
    for edge in edges.iter() {
        if (edge.flow == 0) | (edge.from == start) | (edge.to == goal) {
            continue;
        }

        let ur = edge.from / m;
        let uc = edge.from % m;

        let vr = edge.to / m;
        let vc = edge.to % m;

        if ur < vr {
            ans[ur][uc] = 'v';
            ans[vr][vc] = '^';
        } else if ur > vr {
            ans[ur][uc] = '^';
            ans[vr][vc] = 'v';
        } else if uc < vc {
            ans[ur][uc] = '>';
            ans[vr][vc] = '<';
        } else {
            ans[ur][uc] = '<';
            ans[vr][vc] = '>';
        }
        cnt += 1;
    }

    println!("{}", cnt);
    for a in ans.iter() {
        let a = a
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("{}", a);
    }
}
