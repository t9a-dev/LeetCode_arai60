// Step2b
// 目的: 別の解法を練習
/*
  参考にした解法
  https://github.com/Yoshiki-Iwasa/Arai60/pull/63/changes#diff-2bdcb63933765dbe0404e9a377923fd93b4294a3bf661a8d7aabde92b955958dR58

  所感
  - 分かりづらく感じるという感覚と、単純なfor-loopで実装されているのでシンプルだと思う気持ちが半分くらい。
  問題自体ががパズルのようなものなので、こう感じるのかなと思った。
  - for-loop内のif文の条件を反転させてネストを浅くできるとは思ったものの、このif文の条件を反転させると混乱する感覚があるのでやめた。
*/

pub struct Solution {}
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for pivot_index in (0..nums.len()).rev() {
            for swap_index in (pivot_index..nums.len()).rev() {
                if nums[pivot_index] < nums[swap_index] {
                    nums.swap(pivot_index, swap_index);
                    nums[pivot_index + 1..].reverse();
                    return;
                }
            }
        }
        nums.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_desc_sorted_next_permutation_test() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn step2b_test() {
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
