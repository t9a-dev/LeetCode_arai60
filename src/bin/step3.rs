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
  時間計算量: O(log n)
  空間計算量: O(1)
*/

/*
  1回目: 1分53秒
  2回目: 1分36秒
  3回目: 1分20秒
*/

pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty");
        }

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
    fn step3_test() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![2, 3, 1]), 1);

        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn step3_empty_nums_test() {
        let empty_nums = Vec::new();
        Solution::find_min(empty_nums);
    }
}
