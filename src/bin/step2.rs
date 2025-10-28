// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  他の人のコードを読んで考えたこと
  - 最初から片方の配列先頭ともう片方の配列全部のペアを作ってヒープに乗せるのではなく、それぞれの配列の先頭から一つずつ進めて見ていく方法だと理解した。
  この方法だと結果に必要な個数kに達したら終了する。最小データを常にヒープから取り出すのでサイズも最小に抑えられる。
  それぞれの配列をx,yのような座標に見立てて、より左上が小さくなるという視点であると理解した。
  https://github.com/Yoshiki-Iwasa/Arai60/pull/9/files#diff-911dbcaf9231de51eb4685fb3e690b3584a9cd21c478e732ee7c5b1bcf528eb7R68
  https://github.com/Yoshiki-Iwasa/Arai60/pull/9/files#r1647019606

  - HashSetを使わない方法でもかなりシンプルで分かりやすい実装例だと思った。
  だいぶ時間を使ってしまっているのでこの方式はスキップ
  https://github.com/nanae772/leetcode-arai60/pull/11/files#diff-a77cdd9dedb7558a91428c0fd65e8f740262547dfb0386ce6dea1660498eb50fR1

  他の人のコードの理解
  https://github.com/Yoshiki-Iwasa/Arai60/pull/9/files#diff-911dbcaf9231de51eb4685fb3e690b3584a9cd21c478e732ee7c5b1bcf528eb7
  - 2次元平面として考えたとき左上(0,0)が最小になるので、最小ヒープに(nums1[0]+nums2[0],0,0)を入れておく。(nums1[x],nums2[y],x,y)
  ヒープに追加した座標はHashSetで記録しておいて、一度見た座標はヒープに追加しない。
  最小ヒープから取り出して次の座標のペアと和を最小ヒープに追加する。
    - 次のペアとは右(x+1)と下(y+1)のイメージ
  戻り値はk個のペアがあればよいので、戻り値の配列のサイズがkに達するまで繰り返す。

  改善する時に考えたこと
  - 構造体を定義してフィールドで値を取り回したほうが扱いやすそう。
  - タプルで値を入れるところを型で表現して取り違えなどを防止した。
  - 状態を持つようになったので、繰り返し呼ばれる要件にも対応できるように状態を初期化するメソッドを追加

  所感
  - 記述量は増えたが、保守性は上がっていると思った。
    - 最小ヒープであること、配列のインデックスのペアを型で表現できている。
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Sum(Reverse<i64>);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct PositionPair(usize, usize);

struct TopKSmallestSumPairs {
    nums_x: Vec<i32>,
    nums_y: Vec<i32>,
    visited_positions: HashSet<PositionPair>,
    pairs_heap: BinaryHeap<(Sum, PositionPair)>,
}

impl TopKSmallestSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        if nums1.is_empty() || nums2.is_empty() {
            panic!("nums1 and nums2 must be not empty");
        }

        let mut visited_positions = HashSet::new();
        let mut pairs_heap = BinaryHeap::new();
        let initial_position = PositionPair(0, 0);
        let sum = nums1[0] + nums2[0];

        visited_positions.insert(initial_position.clone());
        pairs_heap.push((Sum(Reverse(sum.into())), initial_position));

        Self {
            nums_x: nums1,
            nums_y: nums2,
            visited_positions,
            pairs_heap,
        }
    }

    // 状態をクリアして初期状態から計算したい時
    pub fn clear_state(&mut self) {
        self.visited_positions.clear();
        self.pairs_heap.clear();

        let initial_position_pair = PositionPair(0, 0);
        let sum = self.nums_x[0] + self.nums_y[0];

        self.visited_positions.insert(initial_position_pair.clone());
        self.pairs_heap
            .push((Sum(Reverse(sum.into())), initial_position_pair));
    }

    pub fn pairs(&mut self, k: usize) -> Vec<Vec<i32>> {
        let mut k = k;
        if self.nums_x.len() * self.nums_y.len() < k {
            k = self.nums_x.len() * self.nums_y.len();
        }

        let mut top_k_smallest_pairs = vec![];

        while let Some((_, position_pair)) = self.pairs_heap.pop() {
            if top_k_smallest_pairs.len() == k {
                break;
            }

            let PositionPair(x, y) = position_pair;

            top_k_smallest_pairs.push(vec![self.nums_x[x], self.nums_y[y]]);
            self.register_pair(PositionPair(x + 1, y));
            self.register_pair(PositionPair(x, y + 1));
        }

        top_k_smallest_pairs
    }

    fn register_pair(&mut self, position_pairs: PositionPair) {
        let PositionPair(x, y) = position_pairs;

        match (
            self.nums_x.get(x),
            self.nums_y.get(y),
            self.visited_positions.insert(position_pairs),
        ) {
            (Some(num_x), Some(num_y), true) => {
                let sum = num_x + num_y;
                self.pairs_heap
                    .push((Sum(Reverse(sum.into())), PositionPair(x, y)));
            }
            _ => (),
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }

        if k <= 0 {
            return vec![];
        }

        let k = k as usize;
        let mut top_k_smallest_sum_pairs = TopKSmallestSumPairs::new(nums1, nums2);
        top_k_smallest_sum_pairs.pairs(k)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 2]]
        );

        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 4),
            vec![vec![1, 1], vec![1, 2], vec![1, 3]]
        );

        let result: Vec<Vec<_>> = Vec::new();
        assert_eq!(
            Solution::k_smallest_pairs(vec![1], vec![1, 2, 3], 0),
            result
        );
    }

    #[test]
    fn step2_clear_state_test() {
        let mut top_k_smallest_sum_pairs = TopKSmallestSumPairs::new(vec![1, 7, 11], vec![2, 4, 6]);

        assert_eq!(
            top_k_smallest_sum_pairs.pairs(3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        assert_ne!(
            top_k_smallest_sum_pairs.pairs(3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );

        top_k_smallest_sum_pairs.clear_state();
        assert_eq!(
            top_k_smallest_sum_pairs.pairs(3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
    }
}
