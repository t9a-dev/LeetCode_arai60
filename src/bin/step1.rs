// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 昇順ソートされた整数を持つ配列num1,num2と整数kが与えられる。
  num1,num2それぞれの要素同士のペアの和を求める。(num1[i],num2[j])
  求めた和の最小k個について和を構成するペアの配列で返す。

  何がわからなかったか
  - 入力のnums1,nums2が非常に大きい場合に全てのペアを探すような処理は時間計算量的に破綻することを見逃してしまった。
  - 入力のnums1,nums2について昇順ソートされているので、全てのペアを見なくてもkの値から捨てることができる入力の数を求められそうだが、具体的な手法がわからなかった。

  何を考えて解いていたか
  - ペアとペアの和を関連付けた状態で、ヒープによるソートでk個取り出す。
  全てのペアの組み合わせを見るので時間計算量がO(N^2)になりそう。ヒープを使うのでO(N^2 log N)になる。
  与えられる配列が昇順ソート済なのがポイントな気がするが、このポイントを活かす具体的な手段が自分の引き出しにない感じがするのでナイーブな実装をする。
  最大ヒープに(sum,(num1,num2))の形で入れていきpeekで見ながら小さい値のみpushしつつ、popで大きい値を捨てる。
  k回ポップして結果を返す。
  ここでLeetCodeに提出して「Time Limit Exceeded」となった。
  入力の配列それぞれと、kが大きい場合を考慮できていなかった。
  kの値からnumsの殆どを捨てられる気がするが、時間を使いすぎなので参考の実装を見ることにした。

  解法の実装を理解する
  https://leetcode.com/problems/find-k-pairs-with-smallest-sums/solutions/3687915/rust-heap-solution/
  - nums1,nums2のサイズを取得しておく。
  nums1を先頭から全走査しながら、タプルで最小ヒープに入れていく。(nums2[0]との和,nums1の位置i,nums2の位置0)
  - k回次の操作を行っている
  最小ヒープからpopして、(ペアの合計,nums1の位置i,j=0)を取り出す。
  結果を返す配列にポップした値のペアを入れる。（初回はnums1[0],nums2[0]となり最小のペアになる。nums1,nums2は昇順ソートされているため。）
  (nums1[i],nums2[j+1]の値),i,j+1)を最小ヒープにプッシュしている。初期化時にnums1をベースにnums2[0]の値とペアを作っているので、nums2[1]とのペアを作っていると理解。
  - まとめ
  最初にnums1の各数値とnums2[0]の値でペアを作って最小ヒープに入れていく。
  最初に最小ヒープからpopする。(nums1[0],nums2[0]の最小ペア)
  nums1の各数値とnums2[1]を最小ヒープにpushしていくことで、それぞれの配列の最小ペアを作っている。
  nums1,nums2が昇順ソートされているのでこの方法になっていると理解した。

  正解してから気づいたこと
  - ペアを作って合計値を見るまではそのペアを捨てて良いのかがわからないのが難しい点だと思った。
*/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if k <= 0 {
            return vec![];
        }

        let k = k as usize;
        let mut smallest_by_sum: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut top_k_smallest_pairs = vec![];

        for (i, num1) in nums1.iter().enumerate() {
            let sum = num1 + nums2[0];
            smallest_by_sum.push((Reverse(sum), i, 0));
        }

        for _ in 0..k {
            if let Some((Reverse(_), i, j)) = smallest_by_sum.pop() {
                top_k_smallest_pairs.push(vec![nums1[i], nums2[j]]);

                if let Some(num2) = nums2.get(j + 1) {
                    let sum = nums1[i] + *num2;
                    smallest_by_sum.push((Reverse(sum), i, j + 1));
                }
            }
        }

        top_k_smallest_pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
    }
}
