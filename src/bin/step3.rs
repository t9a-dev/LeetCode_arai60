// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums.len()
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 1分14秒
  2回目: 0分35秒
  3回目: 0分33秒
*/

pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let original_len = nums.len();
        nums.retain(|v| *v != 0);
        nums.resize(original_len, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let mut before_nums = vec![0, 1, 0, 3, 12];
        let after_nums = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0];
        let after_nums = vec![0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);

        let mut before_nums = vec![0, 0, 1];
        let after_nums = vec![1, 0, 0];
        Solution::move_zeroes(&mut before_nums);
        assert_eq!(before_nums, after_nums);
    }
}
