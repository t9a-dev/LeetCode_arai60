# HashSet初期化方法の違いによる計算時間の見積もり
leet code: https://leetcode.com/problems/unique-email-addresses/description/
review: https://github.com/t9a-dev/LeetCode_arai60/pull/4#discussion_r2422473825

leet code入力の制約として
- 配列長: 1 <= values.length <= 100
- 配列要素の文字列長: 1 <= values[i].length <= 100

## new()に比べwith_capacity(N)で初期化すると、どの程度計算時間が改善するのか。
結論としてはハッシュテーブルのリサイズが発生しても支配的ではないので時間計算量の観点から改善されない。（処理全体の時間計算量`O(N * L)`に対して、ハッシュテーブルリサイズ時の時間計算量は`O(N)`となり支配的ではない。）  
また、正規化後にユニークになる送信先メールアドレスを保持するという用途で考えると、入力に対してサイズが小さくなることが殆どであることから、常に入力と同サイズのメモリを確保しておくのは無駄だと考えられる。

- new()で初期化した場合は要素の追加によりHashSetのサイズを伸張する（メモリ）操作が発生する。
  - capacity上限に当たった場合に既存のサイズを倍にするように伸張する。
    - リサイズ回数は`log N`
    - リサイズ時に要素全ての再配置が行われるので、この操作の時間計算量は`O(N)`
- with_capacity(N)で初期化するとNのサイズでメモリの確保が行われ、insertしても初期化時のサイズを超えなければ伸張操作は発生しない。
  - 今回のケースでは入力サイズを超えることはないので、最初に一度メモリの確保を行うだけで済む。
    - リサイズは行われない


### HashSet::new()
https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.new
初期化時点ではサイズが0でinsert操作が行われるまでメモリの確保が行われない。

### HashSet::with_capacity(N)
https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.with_capacity
引数で指定されたサイズを最小とするHashSetを作成する。初期化時にメモリの確保が行われる。

### 検証コード
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f6fa41229c29bfef3e71a84e1321560d
``` rust
use std::collections::HashSet;
use rand::{distr::Alphabetic, Rng};

fn main() {
    let mut empty_set: HashSet<String> = HashSet::new();
    let mut allocate_count: HashSet<usize> = HashSet::new();
    let mut rng = rand::rng();

    println!("empty_set initialized capacity: {}", empty_set.capacity());

    for i in 0..100 {
        let random_text = (0..100).map(|_| rng.sample(Alphabetic) as char).collect::<String>();
        empty_set.insert(random_text);
        let capacity = empty_set.capacity();
        allocate_count.insert(capacity);
        
        println!("insert {i}回目 capacity: {capacity}");
    }
    
    println!("allocate count: {}", allocate_count.len());
}
```

```
empty_set initialized capacity: 0
insert 0回目 capacity: 3
insert 1回目 capacity: 3
insert 2回目 capacity: 3
insert 3回目 capacity: 7
insert 4回目 capacity: 7
insert 5回目 capacity: 7
insert 6回目 capacity: 7
insert 7回目 capacity: 14
insert 8回目 capacity: 14
insert 9回目 capacity: 14
insert 10回目 capacity: 14
insert 11回目 capacity: 14
insert 12回目 capacity: 14
insert 13回目 capacity: 14
insert 14回目 capacity: 28
insert 15回目 capacity: 28
insert 16回目 capacity: 28
insert 17回目 capacity: 28
insert 18回目 capacity: 28
insert 19回目 capacity: 28
insert 20回目 capacity: 28
insert 21回目 capacity: 28
insert 22回目 capacity: 28
insert 23回目 capacity: 28
insert 24回目 capacity: 28
insert 25回目 capacity: 28
insert 26回目 capacity: 28
insert 27回目 capacity: 28
insert 28回目 capacity: 56
insert 29回目 capacity: 56
insert 30回目 capacity: 56
insert 31回目 capacity: 56
insert 32回目 capacity: 56
insert 33回目 capacity: 56
insert 34回目 capacity: 56
insert 35回目 capacity: 56
insert 36回目 capacity: 56
insert 37回目 capacity: 56
insert 38回目 capacity: 56
insert 39回目 capacity: 56
insert 40回目 capacity: 56
insert 41回目 capacity: 56
insert 42回目 capacity: 56
insert 43回目 capacity: 56
insert 44回目 capacity: 56
insert 45回目 capacity: 56
insert 46回目 capacity: 56
insert 47回目 capacity: 56
insert 48回目 capacity: 56
insert 49回目 capacity: 56
insert 50回目 capacity: 56
insert 51回目 capacity: 56
insert 52回目 capacity: 56
insert 53回目 capacity: 56
insert 54回目 capacity: 56
insert 55回目 capacity: 56
insert 56回目 capacity: 112
insert 57回目 capacity: 112
insert 58回目 capacity: 112
insert 59回目 capacity: 112
insert 60回目 capacity: 112
insert 61回目 capacity: 112
insert 62回目 capacity: 112
insert 63回目 capacity: 112
insert 64回目 capacity: 112
insert 65回目 capacity: 112
insert 66回目 capacity: 112
insert 67回目 capacity: 112
insert 68回目 capacity: 112
insert 69回目 capacity: 112
insert 70回目 capacity: 112
insert 71回目 capacity: 112
insert 72回目 capacity: 112
insert 73回目 capacity: 112
insert 74回目 capacity: 112
insert 75回目 capacity: 112
insert 76回目 capacity: 112
insert 77回目 capacity: 112
insert 78回目 capacity: 112
insert 79回目 capacity: 112
insert 80回目 capacity: 112
insert 81回目 capacity: 112
insert 82回目 capacity: 112
insert 83回目 capacity: 112
insert 84回目 capacity: 112
insert 85回目 capacity: 112
insert 86回目 capacity: 112
insert 87回目 capacity: 112
insert 88回目 capacity: 112
insert 89回目 capacity: 112
insert 90回目 capacity: 112
insert 91回目 capacity: 112
insert 92回目 capacity: 112
insert 93回目 capacity: 112
insert 94回目 capacity: 112
insert 95回目 capacity: 112
insert 96回目 capacity: 112
insert 97回目 capacity: 112
insert 98回目 capacity: 112
insert 99回目 capacity: 112
allocate count: 6
```