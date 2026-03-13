// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  https://github.com/shining-ai/leetcode/pull/58/changes#diff-098c9b2af68878914119b567138d774d79a99fef79253adcb8c9f2aff223aa02R1
  - そもそも手作業でやるにはどうすれば良いかも分かっていなかったので、簡潔に手順が示されていて参考になる。

  https://github.com/usatie/leetcode/pull/2#discussion_r1937025415
    > https://en.cppreference.com/w/cpp/algorithm/next_permutation
    > 一応こちらに参考実装がございます。
  - std::next_permutationという問題名そのままなメソッドがある。iteratorの先頭、末尾の参照を引数として受け取りインプレイスでnext_permutationにしている。
    - next_permutationが存在する場合はtrueを返す。
    - next_permutationが存在しない時はfalseを返す。（昇順ソートが行われる）
  - この実装を見るとupper_boundを使っているので二分探索を活用する方向自体は間違っていなかったが、そもそも手作業で何をしたいのかがはっきりしないまま総当りで条件分岐して対応しようとしていたのが良くなかったと思った。
  直接関係ないが、std::next_permutationのドキュメントで[first,last)という表記を見た時に自然と、left-close,right-openな区間を扱うと理解できていることに気付いてコーディング練習による進歩を感じた。
  コーディング練習を始める前は区間を扱うプログラミングをしたことがなく区間という概念すら知らなかったため。

  https://github.com/olsen-blue/Arai60/pull/59#discussion_r2030531369
    > 平均計算量は、可能な全入力に対しての平均です。たとえば、クイックソートならば、全順列を入れてみて平均を取ります。
    > 償却計算量は、ある最悪な入力の列に対しての振る舞いのことです。
    > たとえば、list に append していくと、たまに、メモリーのリアロケーションで配列長かかりますが、大きさが倍々に増えていく場合(倍でなくても等比級数的ならばよい)は、reallocation のコストは定数で平均すると抑えられます。
    > 何も言わずに計算量と書くと、最悪計算量のことを指すことが多いですね。これは最悪な入力に対しての計算の増え方がある関数で抑えられるということです。
  - 償却計算量について。
  RustのVec型を使う時のことを考える。
    - 要素のpush操作は定数時間O(1)で行える。
    - capacityを超えるような要素の追加が行われたタイミングで確保しているメモリ領域の伸張が行われる。
      - このとき伸張前のVecの要素数をnとすると、伸張後のメモリ領域へのコピーにO(n)の時間計算量を必要とする。
    - ただし、capacityの伸張は指数的に増加するのでn回pushした時のコピーコストの総和がO(n)に収まる。
      - 過去にもらったレビューでcapacityが指数的に増えることを確認したのを思い出した。
        https://github.com/t9a-dev/LeetCode_arai60/pull/4#discussion_r2422473825
        - n回pushしたとき、capacityの上限に当たってメモリ領域の伸張による時間計算量O(n)の時間計算量が発生している。
          - n=3のとき、3回pushすると時間計算量O(3)のコピーが1回発生している。
          つまり、O(3) / 3 = O(1)となりpush操作時の平均時間計算量O(1)となっている。
          ここが「償却計算量」と呼ばれている部分だと理解した。
        - https://github.com/t9a-dev/LeetCode_arai60/pull/4#discussion_r2422473825
            > with_capacity つけてもいいですが、reserve しておくことによる速度改善はどれくらいでしょうか。時間で見積もって欲しいです
          時間で見積もっていなかったので考えてみる。
          コードを見ると正規化したメールアドレスをHashSetにinsertしている。
          入力の制約からメールアドレスは最悪ケースで長さ100のメールアドレスが100個ある。
          capacityが0からスタートしたとき、100回insertするとメモリ領域の伸張は合計6回発生する。
          0 + 3 + 7 + 14 + 28 + 56 = 108
          108 / 10 ^ 8 = 0.00000108s = 1.08μs　となる。
          1.08μsを節約するためにHashSet::with_capacity(emails.len())を理解して書いているのであれば、ソフトウェアエンジニアの常識から外れている行為だなと思った。
          コードの保守性（読み書き）、必要になるかわからないメモリ領域を最初にまとめて確保しているなどトレードオフが見えていればwith_capacityは使わないと理解した。

  https://github.com/olsen-blue/Arai60/pull/59#discussion_r2030548887
  - unreachableな分岐について。Rustではunreachable!()マクロがある。match式で全てのパターンを網羅する必要がある（コンパイルが通らない）ものの、論理的に到達しない条件で使ったことがある。
    https://doc.rust-lang.org/std/macro.unreachable.html

  https://github.com/Yoshiki-Iwasa/Arai60/pull/63/changes#diff-039585a3af25cc10aa49f5c04c849ebd8159420b1ef4145603c86e67235b6ad1R15
  - 解法の考え方が分かりやすい。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/63/changes#diff-2bdcb63933765dbe0404e9a377923fd93b4294a3bf661a8d7aabde92b955958dR57
    > // 本当は、nums.len()-2やnums.len()-1で閉じてもいいが、変にミスりたくないのでnums.len()で閉じる
  - 自分もindex周りは手書きで-1,-2するのは抵抗があるので同じ感覚だなと思った。

  - C++のis_sorted_untilについて。
    https://cpprefjp.github.io/reference/algorithm/ranges_is_sorted_until.html
      > distance(first, last) < 2 なら last を返す。そうでない場合、[first,last] の中でソートされている範囲を [first,i) としたとき、そのイテレータ i を返す。
  Rustにはソート済であるかどうかを判定するメソッドがstd::slice::is_sortedがあった。
    https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted


  参考にした解法
  https://github.com/Yoshiki-Iwasa/Arai60/pull/63/changes#diff-039585a3af25cc10aa49f5c04c849ebd8159420b1ef4145603c86e67235b6ad1R15
  - windows(2).rposition()の動作で少し戸惑った。
  nums=[2,1,3]のとき、
  nums.windows(2) -> [[2,1], [1,3]]
  [[2,1], [1,3]].rposition(|w| w[0] < w[1])
    - [1,3] -> true windows(2)で生成されたイテレータをrpositionで末尾から見ている
    - [2,1] -> false [1,3]でtrueになった時点で早期リターンするので評価されない
  rpositionが扱っているのは[[2,1], [1,3]]のイテレータなので、値のインデックスではなく、windows(2)で生成されたイテレータのindexが返される。
  [1,3]のインデックスは1なのでSome(1)が返ってくる。

  所感
  - rpositionというメソッドを初めて見た。
    https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rposition
  - 実装を見ると内部でtry_rfoldというメソッドを呼び出している。try_rfoldはstd::DoubleEndedIteratorトレイトで定義されている。
    https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#method.try_rfold
      > This is the reverse version of Iterator::try_fold(): it takes elements starting from the back of the iterator.
  - try_*r*foldのrは勝手にrightのrかと思っていたが、reverseのrだった。逆順（右から左）でイテレータを回してくれる。
    DoubleEndedIteratorも初めて見た。両端から要素を取得できるイテレータとのこと。
    https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
  - 問題自体がパズルに見えるので解法もおのずとパズルに見えるのかと思った。
  - 別の解法も練習しておいたほうが良さそう。
    step2a
      https://github.com/olsen-blue/Arai60/pull/59/changes#diff-ee8de8f68c4ae03917be0fa643c8defc8592e248688aed43937777d663dbfccdR168
    step2b
      https://github.com/Yoshiki-Iwasa/Arai60/pull/63/changes#diff-2bdcb63933765dbe0404e9a377923fd93b4294a3bf661a8d7aabde92b955958dR58

*/

pub struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // windows(2)で生成されたイテレータを逆順に走査していく。最初にpredicate(w[0] < w[1])がtrueとなるようなwindowのindexが返される。
        let Some(window_index) = nums.windows(2).rposition(|w| w[0] < w[1]) else {
            // ソート済なのでreverseして終了。
            nums.reverse();
            return;
        };

        let Some(rightmost_successor_index) = nums.iter().rposition(|v| nums[window_index] < *v)
        else {
            unreachable!();
        };

        nums.swap(window_index, rightmost_successor_index);
        nums[window_index + 1..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        let nums = vec![1, 2, 3];
        assert_eq!(nums.windows(2).rposition(|w| w[0] < w[1]), Some(1));

        let nums = vec![2, 1, 3];
        assert_eq!(nums.windows(2).rposition(|w| w[0] < w[1]), Some(1));

        let nums = vec![3, 2, 1];
        assert_eq!(nums.windows(2).rposition(|w| w[0] < w[1]), None);
    }

    #[test]
    fn step2_desc_sorted_next_permutation_test() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn step2_test() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 3, 1]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 1, 2]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![3, 2, 1]);

        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);

        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
