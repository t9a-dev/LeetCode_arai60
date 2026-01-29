// Step1a
// 目的: 別の実装方法で練習(再帰部分をヘルパーメソッドに切り出す)

/*
  問題の理解
  - 整数からなる配列numsが与えられる。numsに含まれる異なる整数から順列を作って返す。
  順列: https://ja.wikipedia.org/wiki/%E9%A0%86%E5%88%97
  要するにnumsに含まれる整数を使って、考えられる全ての並び順の配列を作って返すと理解。
  nums = [1,2]
  output = [[1,2],[2,1]]
  nums = [1,2,3]
  output = [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

  何を考えて解いていたか
  - 再帰処理の設計
    - base_case
      nums.len() == 1のときに、vec![nums]として返す。
    - recursive_case
      nums.pop_frontして値を取っておく。
      numsを引数にrecuursive_caseに入る。
      recursive_caseの戻り値にpop_frontで取り出しておいた値をpushする。
      result配列に詰める。
  単純に再帰処理に書き直せなかったので解答を写経する。

  解答の理解
  https://leetcode.com/problems/permutations/solutions/3797989/video-backtracking-100-unlocking-permuta-8u42/
  - nums[i]を取り除いた配列と、取り除いたnums[i]を利用して順列の配列を作っている。
  - base_caseとして、numsが空になったとき、順列に加えるべき値は存在しないので、順列の集合に順列をを加える。
  - 順列の集合を結果として返している。

  所感
  - 何をしているのか理解しているものの、コードにするまでに距離を感じた。
  - 参考にした解答を読み解いて自分の分かりやすいように書き換えた。
  解答を読むだけでは理解しきれず、写経して処理の流れをメモの上でトレースしたところ、何をしているのか理解できて読みやすいように書き換えることができた。
  他人の書いたコードが読めるかどうかという点だなと思った。
  - 自分だけかもしれないが、nums.len()としているループの中でnums.push_back(num)としていると、一瞬無限ループしそうに見える気持ち悪さがある。
  forの直前で let nums_len = nums.len();として for _ in 0..nums_len としようかと思ったが、これはこれで無駄なことをしているので混乱しそう。
*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        Self::make_permutations(
            VecDeque::from_iter(nums.into_iter()),
            Vec::new(),
            &mut permutations,
        );

        permutations
    }

    fn make_permutations(
        mut nums: VecDeque<i32>,
        permutation: Vec<i32>,
        permutatioins: &mut Vec<Vec<i32>>,
    ) {
        if nums.is_empty() {
            permutatioins.push(permutation);
            return;
        }

        for _ in 0..nums.len() {
            let num = nums.pop_front().unwrap();
            let mut permutation = permutation.clone();

            permutation.push(num);
            Self::make_permutations(nums.clone(), permutation, permutatioins);

            nums.push_back(num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
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
