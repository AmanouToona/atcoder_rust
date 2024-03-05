# rust 提出

## 言語機能関連メモ

- vec の sort は sort_by() or sort_by_key() デフォルトで昇順。 .rev() で反転
- btreeMap: キーの昇順で格納される
- binaryHeap: 降順で pop

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
