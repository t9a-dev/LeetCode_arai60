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
  - 二分探索のコメント集
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0#heading=h.c15qprmvxkc2
  - 35. Search Insert Positionのコメント集
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0#heading=h.e13uiztrq2u9
    - 二分探索関連のコメントが豊富
    https://github.com/Ryotaro25/leetcode_first60/pull/45
    https://github.com/seal-azarashi/leetcode/pull/38
    https://github.com/Fuminiton/LeetCode/pull/41#discussion_r2080995529

    - 値が大きい場合のオーバーフローについて。
    https://github.com/Ryotaro25/leetcode_first60/pull/45#discussion_r1878268512
    問題を解いている時に意識の中になかった。C++でint型の値を扱っている。
    (start + end) / 2 について start + (end - start) / 2 としてオーバーフローを避けるべきという内容。
    endをnums.len()から持ってきている。numsのサイズが非常に大きい場合にendはintの上限まで膨らむ可能性がある。
    つまり、endがINT::MAXみたいな感じになった時に、startを加算するとオーバーフローするという内容だと理解した。
    自分のコードにも当てはまる指摘なので、修正する。
    それにしても、(start + end) / 2 を見た時にオーバーフローするかも知れないと気付けても、start + (end - start) / 2 にしようと思いつけないと思った。
    数学的なテクニックを感じた。(start + end) / 2 を代数変形すると start + (end - start) / 2 で等価になる。今回この方向の考え方に触れられたので、いつか思い出して使えそうだと思った。
    start + (end - start) / 2 でやっていることとして
      - オーバーフローしないよう先に減少する方向の演算を行う。
        - (end - start) / 2
      - 前の項の計算結果は 1 / 2 以下になっており start を加算してもオーバーフローしない。
    start = 1 end = 3 middle = 2
      - (start + end) / 2  -> (1 + 3) / 2 = 4 / 2 = 2
      - start + (end - start) / 2 -> 1 + (3 - 1) / 2 = 1 + 2 / 2 = 1 + 1 = 2
    単純に式として見ると代数変形したところで結果は変わらないが、コンピュータに計算させることを前提とすると結果が変わる（オーバーフローの有無）式になることが面白いと思った。

  - どこか(見失った)のコメントからたどり着いた、ソートされていない大きなデータ10GBの中央値を2GB以内の空間計算量で求めるには？といったstack overflowの質問。
  時間を書けて調べれば解法が何を言っているのかわかりそうな気もするが時間切れなのでメモのみ。
  https://stackoverflow.com/questions/3572640/interview-question-find-median-from-mega-number-of-integers/3576479

  改善する時に考えたこと
  - step1.rsでは閉区間で区間を扱っていたが、半開区間(a..b]で区間を扱うように変更。特に理由が無ければOff-by-oneエラーを避けるために半開区間(left-close,right-open)を使った方が良さそう。
  - middle = (start + end) / 2 についてオーバーフローしないよう middle = start + (end - start) / 2 とする。

  所感
  - Rustではpartition_pointメソッドで同じことができそう。
  https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point
  実装はbinary_search_byメソッドのWrapperという感じ。
*/

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();

        // left-close,right-open
        // [0..1) の時、区間内に値が残っている
        // [0..0) の時、区間内に値が残っていない
        while start < end {
            let middle = start + (end - start) / 2;
            let middle_value = nums[middle];
            if middle_value == target {
                return middle as i32;
            }

            if target < middle_value {
                // 右開区間(right-open)であり、その値自体を含まないのでそのままmiddleを代入
                end = middle;
                continue;
            }

            // 左閉区間(left-close)であり、その値自身を含むのでmiddle自体をスキップするためにmiddle + 1を代入
            start = middle + 1;
        }

        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);

        assert_eq!(Solution::search_insert(vec![], 8), 0);
    }
}
