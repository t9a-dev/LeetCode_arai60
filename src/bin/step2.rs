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
  コメント集と他の人のコードを読んで考えたこと
  https://github.com/sakupan102/arai60-practice/pull/43/changes/BASE..36167fcd716b107e83476e6b2f73821e91d5554e#diff-4ccb56a4631a2bd95d503978df1e47c6cc3893b0b74adc1393bc155f0ce88bc2R2
  - nums[i] > nums[i+1] となる境界を探すという考え方は思いつかなかった。別の解法として二分探索の良い練習になりそう。

  https://github.com/YukiMichishita/LeetCode/pull/7#discussion_r1561132164
  - 二分探索を実装するときの考え方や意識すると良いこと

  https://github.com/Ryotaro25/leetcode_first60/pull/46/changes/BASE..5cd497a61c1610dfb252de6f0dd2a0823e7b2bec#r1869993674
  - odaさんによるオプショナルな質問。自分でも考えてみる。
  Q.> 「2で割る処理がありますがこれは切り捨てでも切り上げでも構わないのでしょうか。」
  A. 良くない。切り捨てにしておかないとend側が動かなくなり無限ループになるので、切り上げは不可。切り捨てにしておくとmiddleは左に寄っていくのでend = middle としても未確定領域が縮小するという感覚。
  Q.> 「nums[middle] <= nums[right] とありますが、これは < でもいいですか。」
  A. 良い。境界を探しているわけではなく、最小値そのものを探しているため。
  Q.> 「nums[right] は、nums.back() でもいいですか。」
  A. 良くない。endポインタを更新して最後尾が移動すること（未確定領域の縮小）を期待しているのでnums.last()では不変条件が壊れて無限ループになるため。
  Q.> 「right の初期値は nums.size() でもいいですか。」
  A. 良くない。自分のコードで、end = nums.len() に書き換えただけでは動かなくなる。配列の添字を見ることを想定して実装している。配列中の最小値を探したいのに配列外を参照しようとすると不変条件が破綻する。

  - 他の人のコードだと、最小値そのものを探すというよりは区間のまとまりの境界自体を探して答えを求めているコードが多いように見える。
  自分がstep1で解答としてみたコードは最小値自体を探すものであったので、step2では境界を探すコードを書いたほうが二分探索の練習になりそう。

  https://github.com/olsen-blue/Arai60/pull/42/changes#r1993813625
    > odaさんのコメントをいただいた上で、再考してみましたが、境界を求める二分探索という整理ができそうであれば、rightの初期値がlen(nums)なのもかなり良いと感じるようになりました。
    > 一方で、欲しいもの一つ見つける二分探索は、rightの初期値はlen(nums) - 1が良いかもという感覚です。
    > 使い分けたいかもしれません。
  - 自分も同じ感想になった。

  odaさんのコメント
  https://github.com/olsen-blue/Arai60/pull/42/changes#r1993204654
    > left を「左側、つまり、条件を満たさないことが判明している左側の最大の場所」として書くこともできます。
    > そうすると、初期値は -1 ですね。
    > この right も「条件を満たさないことが判明している右側の場所の最小」と思うと、right = middle が素直に見えるはずです。

  改善する時に考えたこと
  - 境界を探そうと思って、[start,end)として扱う初期条件を考えてコードを書き始めたがうまく行かなかった。
  そもそも問題で求められている（探したい）のは、配列中に必ず存在する最小値であって、境界ではないことが原因だと思った。
  なので[start,end)のようなend側が配列の外側を指すような区間の設定をすると素直に書けないと思った。
  配列中の最小値を探す時にendが配列の外側を指していると、そこに値は無いのでend側の端点も閉区間にしてしまった方が良いという感覚。
  実装するときの考え方として、境界ではなく配列中の添字と対応する値を見ていると考える方が自然だと思った。

  所感
  - 何か特定の値を探す時は添字で直接配列中の値を見ていく考え方の方が自然だと感じた。
    lower_boundやupper_boundではtargetを境目として配列を分けるイメージなので境界を探すという感覚。
  - [start,end)な半開区間を扱うコードではうまく実装できず方針変更したが別で練習しておく。(step2a.rs)
*/

pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty");
        }

        // [start,end]
        // start <= i <= end
        let mut start = 0;
        let mut end = nums.len() - 1;

        while start < end {
            let middle = start + (end - start) / 2;

            if nums[middle] < nums[end] {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        nums[end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);

        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn step1_empty_nums_test() {
        let empty_nums = Vec::new();
        Solution::find_min(empty_nums);
    }
}
