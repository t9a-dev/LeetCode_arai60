# 計算量の見積もりについて

step3で計算量の見積もりを行おうとしたが、ハッシュマップやソートがコードに出現したときに、どのように見積もればよいかわからなかったので、計算量の見積もり方法について調べた内容をまとめる。  

# 計算量のチートシート
https://www.bigocheatsheet.com/

# 対象のコード
``` rs
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group_anagrams_hash_map: HashMap<String, Vec<String>> =
        HashMap::with_capacity(strs.len());
    for s in strs.into_iter() {
        let mut s_chars = s.to_ascii_lowercase().chars().collect::<Vec<_>>();
        s_chars.sort();
        let sorted_s = s_chars.iter().collect::<String>();

        group_anagrams_hash_map.entry(sorted_s).or_default().push(s);
    }
    group_anagrams_hash_map.into_values().collect()
}
```

# sort()実装について
Rustの[sort()](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)実装では[`driftsort`](https://github.com/Voultapher/driftsort?tab=readme-ov-file)というアルゴリズムが利用されている。時間計算量は`O(N log N)`,空間計算量は`O(N)`  
[イントロソート](https://ja.wikipedia.org/wiki/%E3%82%A4%E3%83%B3%E3%83%88%E3%83%AD%E3%82%BD%E3%83%BC%E3%83%88)で採用されている構造を利用することで、最悪計算量`O(N^2)`となることを回避しているらしい。ほとんどのケースで最悪計算量にならないのになんでそんなに気にするのだろうかと疑問に思ったが、悪意のあるユーザーが入力を調整して最悪計算量となるような入力をしてきたとき(DoS攻撃)に対応するためであるということを知った。既存アルゴリズム採用一つとっても時間・空間計算量だけに着目するだけでなく、セキュリティの観点も関わってくることを知れたのが良かった。

# HashMap実装と操作について
HashMapを操作(参照、検索、追加、削除)の計算量をRustのドキュメントから見つけることができなかった。Rustの[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)はハッシューテーブルというデータ構造を実装したものなので、時間計算量O(1)最悪の場合O(N)、空間計算量O(N)になることが分かった。  
キーの順序がランダムにならないHashMap実装である[IndexMapクレート](https://docs.rs/indexmap/latest/indexmap/)を見かけた。標準ライブラリのHashMapではセキュリティの理由からHashMapのキー生成時にランダムシードを用いてハッシュ化するので結果的に順序がランダムになる。IndexMapでもランダムシードによるハッシュ化は行うが並び順がハッシュに依存せず、操作によって決まるのでテストコードで比較する時や、デバッガーで見たときの値の並び順内容がヒューマンリーダブルになるという利点がある。https://qiita.com/lucidfrontier45/items/cfaec141a45f4fc1480a

# HashMap操作時の時間計算量
HashMap（ハッシュテーブル）の操作はキーが衝突しない場合に計算時間がO(1)になるといろいろな資料に書いてあるが、今回のようにキーに追加する値が固定長でない文字列のような場合は、キーに対して適用されるハッシュ化関数が文字列全ての文字を走査するので文字列長Lとしたとき時間計算量がO(L)が支配的になると理解した。（文字列をキーとして追加時にハッシュ化関数により、時間計算量O(L)。その他の操作は平均でO(1)となる。）

# 所感
- Big-O記法の概要くらいは知っていたが実際に計算量の見積もりしたことがなく、自分ができないことに気付き学習するきっかけになったのが良かった。（これまでコードを見て計算量の見積もりをしたことがなかった。）
- 計算量の見積もりで手が止まることはなくなったが、腹落ちした感じもないので他の問題を進めながら計算量の見積もりも同時に行うことで数をこなして慣れていくしか無いと感じた。
- 計算量の見積もりに伴い、利用しているメソッドの計算量を調べる必要がでてきて標準ライブラリの実装を見に行ったことで、採用されているアルゴリズムやアルゴリズムの周辺知識にふれることができて良かった。[`driftsort`](https://github.com/Voultapher/sort-research-rs/blob/main/writeup/driftsort_introduction/text.md)は最悪計算量になることを避けつつ、コンパイルサイズ、補助空間計算量など様々な点を考慮してバランスが良い形になることを目指していると理解した。
- HashMap(ハッシュテーブル)の操作は平均的にO(1)になると複数の資料にあったので、文字列など固定長でないデータをキーとして追加したときにハッシュ化関数によりO(N)になるのに少し驚いた。（O(1)じゃないの？と思った。）内部的に何をしているのか理解しておくことが重要であると思った。
- 計算量見積もりの学習をしたり、標準ライブラリの実装を見たりしていたのでかなりの時間を使ってしまったと感じた。ただ、LeetCodeを解くことが目的ではなく問題を解く過程でソフトウェアエンジニアとしての常識を身につけることが目的なので計算量の見積もりについての学習に時間を使ったのは間違っていない、などと考えた。