# rust 提出

## 言語機能関連メモ

## Vec

- 重複削除 `v.dedup()` 破壊的

### 四則演算

- saturating_sub: usize の引き算の時に最小値を 0 に固定
- wrapping_add: + !0 を -1 の役割で利用する時に使用

## combination nCk の pow

use ac_library::ModInt998244353 as Mint;

```rust

impl Nck {
    fn new(n: usize) -> Self {
        let mut finv = vec![Mint::new(1); n + 1];
        let mut fac = vec![Mint::new(1); n + 1];
        let mut inv = vec![Mint::new(1); n + 1];

        for i in 2..=n {
            fac[i] = fac[i - 1] * i;
            inv[i] = -inv[998244353 % i] * (998244353 / i);
            finv[i] = finv[i - 1] * inv[i];
        }

        Nck { finv, fac }
    }

    fn get(&self, n: usize, k: usize) -> Mint {
        if n < k {
            return Mint::new(0);
        }
        self.fac[n] * self.finv[k] * self.finv[n - k]
    }
}
```

### sort

- sort_by_key(): x.0 などとして特定の要素でソートする際に利用, `V.sort_by_key(|x| x.0)`
- sort_by(): |x, y| y.cmp(&x) とすると降順ソートにできるなど、無名関数でソートする際に利用 デフォルトで昇順
- .rev() で反転
- sorted ~: vec の非破壊ソート。 ソート後の vec を返す

### binary Heap

- binaryHeap: 降順で pop
- 逆順なら Reverse を使う

```rust
use std::cmp::Reverse;
let mut heap = BinaryHeap::new();
heap.push(Reverse(1));
```

### BtreeSet

2 分探索

```rust
use std::collections::BTreeSet;
use std::ops::Bound::{Included, Unbounded};
#[allow(non_snake_case)]
fn main() {
    let mut tree = BTreeSet::new();

    for i in [1, 3, 5, 6, 8] {
        tree.insert(i);
    }

    for i in 0..=9 {
        // iの次に大きな数を得る
        let upper = tree.range((Included(&i), Unbounded)).next();

        if upper == None {
            println!("{i} no upper");
        } else {
            println!("{i} {:?}", upper);
        }

        // i より小さな最大の数を得る
        let lower = tree.range((Unbounded, Included(&i))).rev().next();
        if lower == None {
            println!("{i} no lower");
        } else {
            println!("{i} {:?}", lower);
        }
    }
}
```

### HashSet

- 挿入は `insert`
- 要素があるかの確認は `is_empty()`

### HashMap

- 各種 itor: key: `keys()`, value: `valus()`, item: `iter()`
- contain_key(): 存在確認

### entry

- and_modify 操作 and_modify(|e: &mut usize| *e = (*e).min(a)) とか

### for

- iproduct!: 多重ループで使用

### float

- 浮動小数の比較はできない [参考](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)

```rust
// Partial orderなものをTotal orderにする
#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

// 上記構造体を使えばソートできる
let mut v: Vec<f64> = vec![0.1, 1.0, 2.0];
// ソートできる！
v.sort_by_key(|&x| Total(x));

// f64のBinaryHeapもできる！
let heap: BinaryHeap<Total<f64>> = BinaryHeap::new();
```

## 文字

### 数字1文字を数字に変換

```rust
let x: usize = x as usize - '0' as usize;
```

### 数字を文字に変換する
```rust
let x: char = (x + '0' as usize) as char;


### Vec<char> を大文字に変換する

```rust
let S: Vec<char> = S
    .into_iter()
    .map(|c| c.to_uppercase().next().unwrap())
    .collect();
```

### 1 文字進める

```rust
let a: cahr = 'a';
let b = char::from_u32(a as u32 + 1).unwrap();
```

### trim

python の strip に当たる  
`trim()`
`trim_matches()`
`trim_end_matches()`
などがあり、 `trim_matches()` `trim_end_matches()` は引数を取れる

## pow

pow の利用時は型が明確であることが必要
`2.pow(2)` ではなく `2i32.pow(2)` など

```rust
fn main() {
    let mut v: Vec<f64> = vec![10.2, 4.6, 0.01, 6.2, 12.0];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", v);
}
```

partial_cmp でもソート可能

## MOD

`rem_euclid` を使う

```rust
ans = ans.rem_euclid(100);
```

### 三角関数

atan(v/h) は v.atan2(h) とすると、h=0 の場合でも動作する。

## 定数

pi `use std::f64::const::PI;`

## AC_library

### union find

Dsu (disjoint set union)

## 文字操作

### char -> num

```
s as usize
```

### num -> char

```
num.to_string();
//std::char::from_digit(s as u32, 10).unwrap()
```

### next_permutation

```
use permutohedron::LexicalPermutation;
S.sort();
loop {

    if !S.next_permutation() {
        break;
    }
}
```

### segtree

practice j.rs 参照
lazy は l.rs 参照
