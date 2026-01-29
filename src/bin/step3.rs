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
  時間計算量: O(n!)
  空間計算量: O(n!)
*/

/*
  1回目: 3分53秒
  2回目: 3分15秒
  3回目: 2分24秒
*/

/*
  所感
  - 計算量の見積もりをする余裕が無くてできていなかったが、この問題は全ての順列を出力する必要があるので、途中の計算を枝刈りすることができず階乗になるのかと思った。
  - GPT-5.2に聞いたところ、問題設定から時間計算量が階乗になることは避けられないものの、アルゴリズムによって定数因数が小さくなるとのことだった。
  https://github.com/Yoshiki-Iwasa/Arai60/pull/55/changes#r1738806743
  ここで実装されているヒープを使った解法では定数因数が小さくなるというメリットがあると理解した。
*/

pub struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        let mut frontier = Vec::new();
        let nums_len = nums.len();

        frontier.push((nums, 0));
        while let Some((mut permutation, swap_index)) = frontier.pop() {
            if swap_index == nums_len {
                permutations.push(permutation.clone());
            }

            for i in swap_index..nums_len {
                permutation.swap(swap_index, i);
                frontier.push((permutation.clone(), swap_index + 1));
                permutation.swap(swap_index, i);
            }
        }

        permutations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let mut expect = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let mut actual = Solution::permute(vec![1, 2, 3]);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        assert_eq!(
            Solution::permute(vec![0, 1]).sort(),
            vec![[0, 1], [1, 0]].sort()
        );
        assert_eq!(Solution::permute(vec![1]), vec![[1]]);
    }
}
