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
  https://github.com/hayashi-ay/leetcode/pull/63/changes#r1537661488
  - backtrackingの定義について。再帰の戻り掛けに何らかの処理を行うこと。
    https://ja.wikipedia.org/wiki/%E3%83%90%E3%83%83%E3%82%AF%E3%83%88%E3%83%A9%E3%83%83%E3%82%AD%E3%83%B3%E3%82%B0

  https://github.com/Yoshiki-Iwasa/Arai60/pull/56/changes#diff-1d48419b0e20772b019b29a3bf3ff9761657623bc3ae1335b2a2f41add6b19a8R8
  - Rust実装のbacktracking

  https://github.com/ryosuketc/leetcode_arai60/pull/40/changes#diff-ad01407803e073f539072a743ce608f45e09a7eb23b9d903e6b81ad196ea9c32R10
  - bit全探索は知らないが、二通りの選択肢だと考えるとbit全探索が適用できるかもしれないという思考の流れになるのかと思った。

  参考した解法の理解
  https://github.com/Yoshiki-Iwasa/Arai60/pull/56/changes#diff-1d48419b0e20772b019b29a3bf3ff9761657623bc3ae1335b2a2f41add6b19a8R8
  - make_subsetsのfor-loopの中で再帰処理をする直前でsubsetにpushして、その後pop()してもとに戻している部分がbacktracking

  所感
  - 答えのコードを見ると何をしているか、どのようにデータが遷移するかは理解できるものの、問題文からこの解法にたどり着くのには距離を感じる。
*/

pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::make_subsets(&nums, &mut Vec::new())
    }

    fn make_subsets(nums: &[i32], subset: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets = Vec::new();
        all_subsets.push(subset.to_vec());

        for (i, num) in nums.iter().enumerate() {
            subset.push(*num);
            all_subsets.extend(Self::make_subsets(&nums[i + 1..], subset));
            subset.pop();
        }
        all_subsets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
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
