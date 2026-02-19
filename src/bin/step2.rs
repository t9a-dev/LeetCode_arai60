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
  https://discord.com/channels/1084280443945353267/1233295449985650688/1242146721094570034
  - step1で時間計算量の見積もりに躓いてかなり時間使ったが、簡単に理解できるものではないという知識があれば避けられたなと思った。
  candidates[i]の最小が1の場合は、分割数の極限の振る舞いというものを知っている必要がある。ただし、ソフトウェアエンジニアの常識には含まれていなさそう。
  今回の問題では制約によりcandidates[i]の最小が2であるので、分割数の極限の振る舞いは関係ないことに注意が必要。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/57/changes
  - Rust実装で解法に幅がある。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/57/changes#diff-d1aabe64d3b8ecf58a421e7472d87dea56ba3ec30721950fd3a93272eed8fa2aR60
  - candidates.split_first()で何をしているのか最初分からなかったが、candidate_indexの代わりになっていると理解した。この方法は思いつかなかった。

  写経した解法
  https://github.com/hayashi-ay/leetcode/pull/65/changes#diff-f084bff8e4dbd771bf8a202d43b499bc30bffb7c10d4c5ccd2102f021910fd19R147
  - 写経してみたが、書きながらcandidate_index + 1 している部分が直感に反すると感じた。
    - 同じ値（同じcandidate_index）はいくつ使っても良いのに、+1するときに特別な判定をしているようには見えないため。
  - candidate_index + 1によって次のcandidatesを見るといったforループのような動きをさせていると理解した。
    - この時点ではsumを変更しておらず、candidate_indexを1つ進めて次に引き継いでいるだけのように見える。
    - candidates[i] + sumがtarget以下のときcandidates[i]を選べて、さらに次でも同じ値(candidates[i])を使える可能性があるのでcandidate_indexはそのまま引き継いでいると理解した。

  改善する時に考えたこと
  - 参考にしたコードはPythonで書かれており、インナーファンクションで1つ外側の変数を見に行けるので再帰関数の引数部分がすっきりとしている。
  - Rustでそのまま書くと引数の数が6つになり、流石に気になるので改善できるか考えてみる。(step2a.rs)
*/

pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination = Vec::new();
        let mut all_combinations = Vec::new();
        Self::make_combinations(
            &candidates,
            0,
            0,
            target,
            &mut combination,
            &mut all_combinations,
        );

        all_combinations
    }

    fn make_combinations(
        candidates: &[i32],
        candidate_index: usize,
        sum: i32,
        target: i32,
        combination: &mut Vec<i32>,
        all_combinations: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            all_combinations.push(combination.to_vec());
            return;
        }
        if candidates.len() <= candidate_index {
            return;
        }

        Self::make_combinations(
            candidates,
            candidate_index + 1,
            sum,
            target,
            combination,
            all_combinations,
        );

        if sum + candidates[candidate_index] <= target {
            combination.push(candidates[candidate_index]);
            Self::make_combinations(
                candidates,
                candidate_index,
                sum + candidates[candidate_index],
                target,
                combination,
                all_combinations,
            );
            combination.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let mut expect = vec![vec![2, 2, 3], vec![7]];
        let mut actual = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        let mut actual = Solution::combination_sum(vec![2, 3, 5], 8);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let expect = Vec::<Vec<i32>>::new();
        let actual = Solution::combination_sum(vec![2], 1);
        assert_eq!(expect, actual);
    }
}
