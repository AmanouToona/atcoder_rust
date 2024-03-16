use ac_library::lcp_array;
use ac_library::suffix_array;
use proconio::input;

fn main() {
    input! {
        S: String,
    }

    let sa = suffix_array(&S);
    let lcp = lcp_array(&S, &sa);

    let mut ans = S.len() * (S.len() + 1) / 2;

    for dup in lcp.iter() {
        ans -= dup;
    }

    println!("{}", ans);
}
