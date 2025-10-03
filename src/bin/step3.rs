// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// MEMO
// step2で他の解答例を見たときにハッシュテーブルを利用したワンパスアルゴリズムを見つけたのでこれで実装した。
// nums_hash_map.removeとしているのは、戻り値のvalueの参照ではなく所有権を得るため。
// nums_hash_map.getとすると戻り値で参照が帰ってきて、try_into()するためにclone()する必要があり、払う必要のないコストがかかる。
// 今回の実装ではペアのnumを見つけた時点でreturnするのでremoveしても以降のコードに影響が無い。

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
            if let Some(complement_index) = nums_hash_map.remove(&complement) {
                return vec![complement_index.try_into().unwrap(), i.try_into().unwrap()];
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
    fn step3_test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
