// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums.len
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 2分48秒
  2回目: 2分17秒
  3回目: 2分11秒
*/

/*
  所感
  - こうしたらこうなるという感覚で覚えて書いている。
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
            unreachable!();
        };

        nums.swap(rfind_first_decreasing_index, swap_index);
        nums[rfind_first_decreasing_index + 1..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_desc_sorted_next_permutation_test() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn step3_test() {
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
