#![recursion_limit = "512"]
use proconio::{input, marker::Usize1, source::line::LineSource};
#[allow(non_snake_case)]

fn main() {
    let stdin = std::io::stdin();
    let mut source = proconio::source::line::LineSource::new(stdin.lock());

    input! {
        from &mut source,
        (_, L, R): (usize, i64,i64)
    }

    let mut ans = 0;

    let mut L = L;
    let R = R + 1;
    let mut q = Vec::new();
    while L != R {
        let mut i = 0;
        while L % 2i64.pow(i + 1) == 0 && L + 2i64.pow(i + 1) <= R {
            i += 1;
        }
        let j = L / 2i64.pow(i);
        println!("{} {}, {} {}", L, L + 2i64.pow(i), i, j);
        L += 2i64.pow(i);

        let l = 2i64.pow(i) * j;
        let r = 2i64.pow(i) * (j + 1) - 1;
        // println!("{} {}", l, r);
        q.push((l, r));

        // input! {
        //     from &mut source
        //     a: usize,
        // }

        // ans += a;
        // ans %= 100;
    }

    // for (l, r) in q {
    //     println!("{} {}", l, r);
    //     input! {
    //         from &mut source
    //         a: usize,
    //     }
}

// println!("{}", ans);

/*
L, R = map(int, input().split())
ans = []
while L != R:
    i = 0
    while L % pow(2, i+1) == 0 and L+pow(2, i+1) <= R:
        i += 1
    ans.append([L, L+pow(2, i)])
    L += pow(2, i)
print(len(ans))
for l, r in ans:
    print(l, r)
*/
// }
//
