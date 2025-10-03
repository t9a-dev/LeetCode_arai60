// Step2_1
// 目的: レビューに基づいて書き直す

/*
  MEMO
  レビューに基づいて修正を行った。
  if の条件文から不要な括弧を削除
*/

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num_i) in nums.iter().enumerate() {
            //　ペアを見たいので同じインデックスはskip()で飛ばす
            for (j, num_j) in nums.iter().enumerate().skip(i + 1) {
                if num_i + num_j == target {
                    return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_1_test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
