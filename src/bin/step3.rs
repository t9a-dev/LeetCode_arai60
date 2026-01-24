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
  1回目: 4分50秒
  2回目: 4分01秒
  3回目: 3分15秒
*/

pub struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;

        if nums.is_empty() {
            return NOT_FOUND;
        }

        let sentinel_subarray_length = nums.len() + 1;
        let mut min_subarray_length = sentinel_subarray_length;
        let mut prefix_sum = nums[0];
        let (mut start, mut end) = (0, 0);

        while start <= end && end < nums.len() {
            if target <= prefix_sum {
                min_subarray_length = min_subarray_length.min((end - start) + 1);
                prefix_sum -= nums[start];
                start += 1;
            } else {
                end += 1;
                if end < nums.len() {
                    prefix_sum += nums[end];
                }
            }
        }

        if min_subarray_length == sentinel_subarray_length {
            return NOT_FOUND;
        }

        min_subarray_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }
}
