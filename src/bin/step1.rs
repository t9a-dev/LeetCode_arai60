// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

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
  - 手作業でやることを考えてみたが規則性が見えず手が止まったので解答を見る。

  何がわからなかったか
  - ナイーブな実装も思いつかなかった。

  解法の理解
  NeetCodeの解説動画
  https://www.youtube.com/watch?v=s7AvT7cGdSo
  - 決定木として見ている。
  - 入力の配列から値をpop_front()してrecursive_caseに入る。配列の長さが1の時をbase_caseとして、配列をそのまま返す。
  - pop_front()した値を戻り値の配列にpush()していく。
  nums = [1,2,3]
  nums = [2,3] pop 1
  nums = [3] pop 2
  nums = [3,2] push 2
  nums = [3,2,1] push 1

  正解してから気づいたこと
  - 再帰処理のたびに let mut result = Vec::new() してVectorを生成しているのが気になるので、一度だけresultを確保して可変参照で取り回した方が良さそう。

  所感
  - Backtrackingは初めてなのでややこしく感じる。慣れの問題だと思うのであまり気にしないようにする。
  - 再帰部分をヘルパーメソッドに切り出して実装の練習をする(step1a.rs)
*/

use std::collections::VecDeque;

pub struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if nums.len() == 1 {
            return vec![nums];
        }

        let mut nums = VecDeque::from_iter(nums.into_iter());
        for _ in 0..nums.len() {
            let Some(num) = nums.pop_front() else {
                continue;
            };
            let mut permutes = Self::permute(nums.clone().into());

            for permute in &mut permutes {
                permute.push(num);
            }
            result.extend(permutes);
            nums.push_back(num);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]).sort(),
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
            .sort()
        );

        assert_eq!(
            Solution::permute(vec![0, 1]).sort(),
            vec![[0, 1], [1, 0]].sort()
        );
        assert_eq!(Solution::permute(vec![1]), vec![[1]]);
    }
}
