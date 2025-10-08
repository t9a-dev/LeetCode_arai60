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
  - if文周りの書き方が冗長なので、もう少しすっきり書ける方法がある

  他の人のコードを読んで考えたこと
  - HashSetを利用したワンライナーの例。mapで参照を返すだけの部分は書き直せそう。
  https://leetcode.com/problems/intersection-of-two-arrays/solutions/4406232/one-liner-using-set-operations/

  - なぜbinary_searchを使っているのか、HashSet自体は使っているのに、filterでの重複判定に使っている理由などよくわからなかった。
  Runtime Beats 100%であることをタイトルで示しているが、step1.rsの自身の実装でもRuntime Beats 100%となるので、この解法は選択肢に入らないなと思った。
  https://leetcode.com/problems/intersection-of-two-arrays/solutions/5678828/beats-100-in-runtime/

  改善する時に考えたこと
  - メソッド名と同名の変数名とならないようintersectedに変更
  - nums1とnums2でサイズが小さい方を全走査に利用する。もう片方はHashSetに変換する。
  Vec<i32>.contains() -> HashSet<i32>.contains()に変更することで時間計算量を改善する。
  - そもそもRustではHashSet::intersection()メソッドが利用できるので、step3.rsではこの方法で書く。
*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let linear_search_base;
        let others: HashSet<i32>;
        let mut intersected: HashSet<i32> = HashSet::with_capacity(nums1.len().min(nums2.len()));

        // 入力のデータ型がi32と固定長なので、HashSet作成時の時間計算量はO(1)
        if nums1.len() <= nums2.len() {
            linear_search_base = nums1;
            others = HashSet::from_iter(nums2);
        } else {
            linear_search_base = nums2;
            others = HashSet::from_iter(nums1);
        }

        // ここで全走査するので、サイズが小さい方が効率が良い
        for base in linear_search_base.into_iter() {
            if others.contains(&base) {
                intersected.insert(base);
            }
        }

        intersected.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut result = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort();
        assert_eq!(result, vec![2]);

        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
