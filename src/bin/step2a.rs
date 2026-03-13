// Step2a
// 目的: 別の解法を練習

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  参考にした解法
  https://github.com/olsen-blue/Arai60/pull/59/changes#diff-ee8de8f68c4ae03917be0fa643c8defc8592e248688aed43937777d663dbfccdR168

  所感
  - 参考にした解法のとおりに関数に分けようかと思ったが、書いていてわかりづらくなるように思ったので、変数名だけ参考にした形になった。
    - innner functionなので問題は無いものの、再利用できるような形(inner function)でunreachableとなっていると少し分かりづらいという感覚。
    - pivot_index -> rfind_first_decreasing_index
*/

pub struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let Some(rfind_first_decreasing_index) = nums.windows(2).rposition(|w| w[0] < w[1]) else {
            nums.reverse();
            return;
        };
        let Some(swap_index) = nums
            .iter()
            .rposition(|v| nums[rfind_first_decreasing_index] < *v)
        else {
            unreachable!()
        };

        nums.swap(rfind_first_decreasing_index, swap_index);
        nums[rfind_first_decreasing_index + 1..].reverse();
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
    fn step2a_desc_sorted_next_permutation_test() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn step2a_test() {
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
