// Step2a
// 目的: 別の解法を練習する

/*
  所感
  - step2の実装を再帰からスタックを利用する方法に書き換えた。入力のnumsを入れ替えながら新しい順列の値を作るという考え方にswapによる処理が合っているように感じる。
  - nums.len()で外側のnumsから毎回長さを算出するには、frontier.push(nums.clone())とする必要があるので、nums_len変数に取り出すようにした。
  要するにループの中でnums.len()をするためだけに、nums.clone()とするのは気が引けるという感覚。
  numsはfrontierにpushしたあとは使わないので、所有権をムーブしてしまった方が良いと考えた。
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
    fn step2a_test() {
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
