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
  時間計算量: O(n * 2 ^ n)
  空間計算量: O(n * 2 ^ n)
  分からなかったのでGPT-5.2に聞いた。
  部分集合の総数は要素数nに対して 2 ^ nとなる。
  subset.clone()をループの中で毎回行っているのでO(n)が追加でコストが掛かっている。
  よって、O(n * 2 ^ n)となる。
*/

/*
  1回目: 2分19秒
  2回目: 2分10秒
  3回目: 2分01秒
*/

/*
  所感
  - 答えを見ずに書き始めたら、これまでのstepと少し違う実装になった。
  再帰処理の外側で一度だけVecを確保して後は可変参照で取り回しているので、少し効率の良い実装になったと思った。
    - 再帰関数の中で毎回Vecを確保していない。
    - subset.clone()は結果のall_subsetsに追加するときにのみ行っている。
*/

pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets = Vec::new();
        let mut subset = Vec::new();

        Self::make_subset(&nums, &mut subset, &mut all_subsets);

        all_subsets
    }

    fn make_subset(nums: &[i32], subset: &mut Vec<i32>, all_subsets: &mut Vec<Vec<i32>>) {
        all_subsets.push(subset.clone());

        for i in 0..nums.len() {
            subset.push(nums[i]);
            Self::make_subset(&nums[i + 1..], subset, all_subsets);
            subset.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let mut subsets = Solution::subsets(vec![1, 2, 3]);
        let mut expect = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        let mut subsets = Solution::subsets(vec![3, 2, 4, 1]);
        let mut expect = vec![
            vec![],
            vec![3],
            vec![2],
            vec![2, 3],
            vec![4],
            vec![3, 4],
            vec![2, 4],
            vec![2, 3, 4],
            vec![1],
            vec![1, 3],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 4],
            vec![1, 3, 4],
            vec![1, 2, 4],
            vec![1, 2, 3, 4],
        ];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        let mut subsets = Solution::subsets(vec![0]);
        let mut expect = vec![vec![], vec![0]];
        subsets.iter_mut().for_each(|x| x.sort());
        subsets.sort();
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        assert_eq!(subsets, expect);

        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    }
}
