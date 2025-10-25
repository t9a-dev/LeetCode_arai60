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
  - BinaryHeapという知らないデータ構造がでてきたので調べる。
  https://leetcode.com/problems/kth-largest-element-in-a-stream/solutions/1924742/rust-8ms-clean-solution/

  優先度付きキューと呼ばれるものである。
  https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

  ヒープと聞くと可変長のデータを置く「ヒープ領域」を連想したが別物である。
  https://e-words.jp/w/%E3%83%92%E3%83%BC%E3%83%97.html

  データ構造の文脈でのヒープはルート要素から子要素に向かって常に等しいか大きく（または常に等しいか小さく）なるという特徴を持っている。
  つまりソートされた状態が維持されるデータ構造であると理解した。

  - 最小ヒープを自前で実装している人を見かけたが、今の自分には負荷が高すぎる気がするので再実装は見送る。
  https://github.com/nanae772/leetcode-arai60/pull/9/files#diff-658181431805723e1620777756da7cdb58506c82abf7524090b7d44eb9506456R20

  - 初期化時にkを-1するのか、addの中で毎回k-1として表すかについてのコメント
    - 個人的には初期化時にk-1しておいて、以降は忘れてしまうのが良いかなと思った。
    構造体に対して実装されているメソッドが参照するフィールドは正しく初期化されていることを期待しても良いだろうといった感覚だと思う。
    addの中で-1するのは良くないと断言できるほど強い意見は持っていないので好みの範囲かなと思った。
  https://github.com/skypenguins/coding-practice/pull/23/files#r2348744706

  - mapを使った方法もあるらしいが、読んでもよくわからなかったのと、時間を使いすぎているので今回は見送る。
  https://github.com/5103246/LeetCode_Arai60/pull/8/files#diff-763b3c6f3f0640ac227a784b20468ee384e947f598c2cf7c9f8fae0a8ed9e9a8R132

  実装例コードの理解
  - ヒープを利用して実装している。
  Rustにおいてstd::collections::BinaryHeapは最大ヒープ（ルート要素が一番大きい）であり、プッシュ時に値をstd::cmp::Reverseでラップすることで最小ヒープとなる。
  https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html

  - 初期化時に引数で受け取ったnumsをReverseでラップして追加することで最小ヒープを生成している
  - 最小ヒープを生成したあとにヒープのサイズが`k`より大きい間popし続けているが、なぜこうしているのかがすぐに理解できなかった。
    - 最小ヒープで生成しているのでk番目以降の値は必要ない。ヒープのサイズは`k`あれば良いのでpopしていると理解した。
    ヒープのサイズが`k`になるまで値を取り出す(pop)と次に取り出す値がk番目の要素になるのでpeek()で取り出さずに参照して値を返している。

  改善する時に考えたこと
  - LeetCodeの採点システム上newメソッド引数のkはi32型である必要がある。構造体のフィールドの型は変えられるのでkをusizeにしておいて、初期化時に変換するようにした。
  - 参考にした実装についてnewメソッドにwhile文を書いてheap.pop()を実行しているコードを見かけた。
    - おそらく初期化してすぐに必要ない値を捨てておくことでaddによる値の追加時にheapサイズの伸張が起きないようにしているのかなと思った。
    確かに合理的ではあるが、newメソッドの中でこれをやると、with_capcityで事前にサイズを確保しておくのと同じようなノイズになる気がするので、このコードは取り除いた。
  - step1でやっていたaddメソッド先頭での早期リターンはやめた。ボトルネックではないため。
  - テストコードでKthLargestをインスタンス化したときの変数名をscore_manager -> kth_largest_finderに変更した。
  違和感ありつつ適当に命名していたので、ChatGPT(GPT-5)に相談して選んだ。

  所感
  - 自身のナイーブな実装と比べて数百倍速度が出ている。(LeetCode実行環境の実行速度が不安定なので数百倍とした)
  経験として残るので不格好でもナイーブな実装をして実行速度が遅いことに向き合うのも大切だなと思った。
*/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        if k <= 0 {
            panic!("k must be 1 or greater");
        }

        Self {
            min_heap: BinaryHeap::from_iter(nums.into_iter().map(Reverse)),
            k: k as usize,
        }
    }

    pub fn add(&mut self, num: i32) -> i32 {
        self.min_heap.push(Reverse(num));

        while self.min_heap.len() > self.k {
            self.min_heap.pop();
        }

        self.min_heap
            .peek()
            .map(|Reverse(v)| *v)
            .unwrap_or(i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_case1_test() {
        let mut kth_largest_finder = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest_finder.add(3), 4);
        assert_eq!(kth_largest_finder.add(5), 5);
        assert_eq!(kth_largest_finder.add(10), 5);
        assert_eq!(kth_largest_finder.add(9), 8);
        assert_eq!(kth_largest_finder.add(4), 8);
    }

    #[test]
    fn step2_case2_test() {
        let mut kth_largest_finder = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kth_largest_finder.add(2), 7);
        assert_eq!(kth_largest_finder.add(10), 7);
        assert_eq!(kth_largest_finder.add(9), 7);
        assert_eq!(kth_largest_finder.add(9), 8);
    }
}
