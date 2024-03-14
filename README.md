# atcoder rust

## メモ

### 四則演算

- saturating_sub: usize の引き算の時に最小値を 0 に固定
- wrapping_add: + !0 を -1 の役割で利用する時に使用

### sort

- sort_by_key(): x.0 などとして特定の要素でソートする際に利用
- sort_by(): |x, y| y.cmp(&x) とすると降順ソートにできるなど、無名関数でソートする際に利用
- sorted ~: vec の非破壊ソート。 ソート後の vec を返す

### for

- iproduct!: 多重ループで使用
