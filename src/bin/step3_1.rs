// Step3_1
// 目的: レビューに基づいて書き直す

// MEMO
// レビューに基づいて修正を行った。
// nums_hash_mapから値を取り出すときにremove()ではなく、get()を利用する
// get()の戻り値は参照なのでクローン操作が必要だが、primitive型(usize)なのでclone()によるコストは無視できると考える

use std::collections::HashMap;

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/
pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hash_map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            let complement = target - num;
            if let Some(complement_index) = nums_hash_map.get(&complement) {
                return vec![
                    // clone()でも良いが、to_owned()の方がやりたいこと（所有権が欲しい）と一致しているので
                    complement_index.to_owned().try_into().unwrap(),
                    i.try_into().unwrap(),
                ];
            }
            nums_hash_map.insert(num, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_1_test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
