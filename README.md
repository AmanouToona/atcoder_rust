# rust 提出

## 言語機能関連メモ

## Vec

- 重複削除 `v.dedup()` 破壊的

### 四則演算

- saturating_sub: usize の引き算の時に最小値を 0 に固定
- wrapping_add: + !0 を -1 の役割で利用する時に使用

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
std::char::from_digit(s as u32, 10).unwrap()
```
