# rust 提出

## 言語機能関連メモ

### 四則演算

- saturating_sub: usize の引き算の時に最小値を 0 に固定
- wrapping_add: + !0 を -1 の役割で利用する時に使用

### sort

- sort_by_key(): x.0 などとして特定の要素でソートする際に利用
- sort_by(): |x, y| y.cmp(&x) とすると降順ソートにできるなど、無名関数でソートする際に利用 デフォルトで昇順
- .rev() で反転
- sorted ~: vec の非破壊ソート。 ソート後の vec を返す

### binary Heap

- binaryHeap: 降順で pop

## HashMap

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
