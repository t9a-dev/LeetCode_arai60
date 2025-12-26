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
  1回目: 2分26秒
  2回目: 1分51秒
  3回目: 2分18秒
*/

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let middle = start + (end - start) / 2;
            let middle_value = nums[middle];
            if middle_value == target {
                return middle as i32;
            }

            if target < middle_value {
                end = middle;
                continue;
            }

            start = middle + 1;
        }

        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3], 2), 1);

        assert_eq!(Solution::search_insert(vec![], 8), 0);
    }
}
