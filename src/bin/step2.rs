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
  講師陣はどのようなコメントを残すだろうか？
  - ヒープ意外のソートアルゴリズムによる実装をしてみた方が良いというコメントをもらいそう。

  他の人のコードを読んで考えたこと
  - LeetCodeのRust実装例ではstructを定義せずにHashMapのentryを直接sort_byでソートしている実装例が多かった。
  https://leetcode.com/problems/top-k-frequent-elements/solutions/3550299/rust-heap-solution/
  この方法だとHashMapのentryをソートするときにソートキーの指定の仕方が、v.0(entryのkey),v.1(entryのvalue)になるので、構造体を定義した自分の実装の方が読みやすいと思った。
  forでHashMapのkey,valueを取り出せることを知ったのでこの解法が構造体を使わない場合のベターな実装になると思った。

  - Pythonにmost_commonというそのまま今回の解法になるメソッドがある。数学的な操作？を行うメソッドが充実しているのはさすがPythonだと思った。
  https://github.com/nanae772/leetcode-arai60/pull/10/files#diff-eb59810221bd66124d828cf3cb0a5d4c0a72b8c00708bc826eeaa4341929422aR16-R17
    - Rustでは外部クレートでitertoolsのcountsメソッドで実現できる。
    https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.counts

  - 自分も与えられた配列は動的に変更されないので、もしかしたらヒープを使う必要はなくて、一度ソートしてしまえばそれでよいのではと思ったものの、コメントのように深く考えていなかったので感心した。
  https://github.com/nanae772/leetcode-arai60/pull/10/files#diff-eb59810221bd66124d828cf3cb0a5d4c0a72b8c00708bc826eeaa4341929422aR46-R47

  - 自分はヒープによるソートで並べ替えたが、この問題の論点としてソートのアルゴリズムでどれを採用するかみたいな点を見ている人が多いように感じた。
  ソートアルゴリズムを実装して見るほど余裕がないのでスキップ
  https://github.com/Apo-Matchbox/LeetCode_Practice/pull/22/files#diff-013dde8937de27960afd2d09c6325ebf53eb00c65d9d2772bc598e798bcdb877R66-R70
  https://github.com/docto-rin/leetcode/pull/9/files#diff-2d1f664c4bc1102fa070db0b0b8329b974594503435ab3437b253c140ebb78f1R109
  https://github.com/yas-2023/leetcode_arai60/pull/9/files
  https://github.com/kt-from-j/leetcode/pull/15/files#diff-a323882b92b7f12828fc115f855f23e9e991aeeddff9612982207b144873c928R90

  改善する時に考えたこと
  - 最小ヒープ方式にしてヒープに対するプッシュ操作の時間計算量を削減する。(ChatGPT(GPT-5)によるレビュー)
    最小ヒープ方式とはヒープのサイズをなるべく小さく保つことで、時間計算量と空間計算量を抑える手法。
    上位から数えてk番目の値を計算する場合に、上位k番目以降の要素は必要ないので捨てていく。
    ヒープのサイズMとした場合に、ヒープに対するpush操作の時間計算量O(log M)、空間計算量O(M)となる。
    ヒープのサイズをkにしておくと、ヒープに対するpush操作の時間計算量O(log k)、空間計算量(k)となる。
    ヒープ操作の時間計算量はヒープサイズに比例するので、サイズはなるべく小さくしておきたいと理解した。
  - 構造体にしておかないと、HashMapから取り出した値の取り回しがしづらい（v.0(key),v.1(value)）と思っていたが、for文でタプルで取り出せることに気づいたので構造体なしで実装してみる。
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct Solution {}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k <= 0 {
            return vec![i32::MIN];
        }

        let k = k as usize;
        let mut frequency_by_value: HashMap<_, _> = HashMap::new();
        let mut top_k_heap: BinaryHeap<Reverse<(_, _)>> = BinaryHeap::new();

        for num in nums {
            frequency_by_value
                .entry(num)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }

        for (value, frequency) in frequency_by_value {
            let entry = Reverse((frequency, value));

            if top_k_heap.len() < k {
                top_k_heap.push(entry);
                continue;
            }

            if let Some(Reverse((peeked_frequency, _))) = top_k_heap.peek() {
                if frequency > *peeked_frequency {
                    top_k_heap.pop();
                    top_k_heap.push(entry);
                }
            }
        }

        top_k_heap.into_iter().map(|Reverse((_, v))| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);

        let mut result = Solution::top_k_frequent(vec![1], 1);
        result.sort();
        assert_eq!(result, vec![1]);

        let mut result = Solution::top_k_frequent(vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}
