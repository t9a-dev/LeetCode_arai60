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
  - まずはナイーブを実装して、そこから改善点を考えていく形の方が自然だという考え方に同意できると思った。
  step1で解法が思いつかず、たまたま見た解法がこの問題の最適解（カダネのアルゴリズム）だったので、ナイーブな実装を一度自分でやってみたほうが良さそうだと思った。
  step2の前にstep1a_naive.rsで総当りのコードを書いてみる。
  https://github.com/docto-rin/leetcode/pull/37/files#diff-52347f58ca1366895729356e24b384b058a947b881fb21e28e844969b8396244R133

  - 二分探索を利用しているように見えるが理解しきれない。
  解法のバリエーションということでメモ。
  https://github.com/nanae772/leetcode-arai60/pull/31/files#diff-77c0c8483d0ad8a4c4bb927c69817643a478d0aafd8c1d2a92edc4b336cca3a3
  二分探索ではなく、分割統治法というらしい。
  https://github.com/garunitule/coding_practice/pull/32/files#diff-5773de699c600fdf54940c38b7799b23dd0c335acf25ddce1b93c7335404131aR65

  コメント集
  - https://discord.com/channels/1084280443945353267/1206101582861697046/1207949240316338176
  かっこいい公式を思いつこうと数時間使ったり、かっこういい公式の振り回し方を考えるのに時間を使うことに余り意味は無いと理解した。
  わからないことを考え続けてもしょうがないのでとっとと解法をみて、解法の理解に時間を使った方が良いと理解した。この点はしっかりと意識して時間を無駄にしないようにしたい。

  - step1.rsはカダネのアルゴリズムと言われるものであるそう。
  "https://en.wikipedia.org/wiki/Maximum_subarray_problem#Kadane's_algorithm"
  https://discord.com/channels/1084280443945353267/1206101582861697046/1207405733667410051

  改善する時に考えたこと
  - numsは冒頭でチェックしているので、max_sum,sumの初期値にnums[0]を入れる。iter().skip(1)で先頭をスキップする。
*/

pub struct Solution {}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty.");
        }

        let mut max_sum = nums[0];
        let mut sum = nums[0];

        for num in nums.into_iter().skip(1) {
            sum = num.max(sum + num);
            max_sum = max_sum.max(sum);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::max_sub_array(vec![-1, -2]), -1);
        assert_eq!(Solution::max_sub_array(vec![1, -1, 1]), 1);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![0]), 0);
    }

    #[test]
    #[should_panic(expected = "nums must not be empty.")]
    fn step2_panic_test() {
        Solution::max_sub_array(vec![]);
    }
}
