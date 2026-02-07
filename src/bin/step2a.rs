// Step2a
// 目的: 再帰処理をスタックの解法に書き換える練習を行う

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  所感
  https://github.com/Yoshiki-Iwasa/Arai60/pull/56/changes#r1741301219
    > ちょっとよく分かっていないんですが、ここの clone は不要ですか?
    > push pop で戻しているということは。
  - ここのコメントは自分の書いたコードにも当てはまる気がするが、cloneは必要な気がする。
  ある時点のsubsetにnums[i]をpushした状態は独立してall_subsetsに加える必要があるという理解のため。
  GPT-5.2に聞いてみたところ、subsetをスタックに積む方法ではclone()は避けられなさそうだった。
  処理を大幅に書き換えれば、for-loopの中のsubset.clone()はなくせそうだったが冗長すぎて書きたくないなと思った。
  - step2ではsubset.clone()に当たる部分は可変参照で取り回しているのでclone()していないものの、all_subsetsを毎回確保しているのでコストは結局同じになっていると思った。
*/

pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut all_subsets = Vec::new();
        let mut frontier = Vec::new();

        frontier.push((nums.as_slice(), vec![]));
        while let Some((nums, mut subset)) = frontier.pop() {
            all_subsets.push(subset.clone());

            for i in 0..nums.len() {
                subset.push(nums[i]);
                frontier.push((&nums[i + 1..], subset.clone()));
                subset.pop();
            }
        }

        all_subsets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
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
