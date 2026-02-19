// Step2b
// 目的: 別の解法を練習する。再帰->ループ

/*
  所感
  - 再帰よりもすっきりと書けるなと思った。
  - combinationをスタックに積む時にclone()するのをためらってどうにかできないか迷ったが、どうにもならないことに気付くのに少し時間を使った。具体的には可変参照を取り回してどうにかならないかと考えた。
  コーディング練習ではコピーは避けられないか？といったことを考えながら実装するのは大切だが、一方で最適化に囚われすぎて動くものを作るといった本来の目的から逸脱しないように気をつける必要があるななどと考えていた。

  > Rustでは超クールなメモリ確保なしのゼロコピーアルゴリズムが安全に書けるからと言って、すべてのアルゴリズムを超クールにゼロコピーでメモリ確保なしに書く必要はない
  https://users.rust-lang.org/t/feeling-rust-is-so-difficult/29962/15
  「Effective Rust 項目20: 過剰な最適化の誘惑を退けよう」より引用
*/

pub struct Solution {}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut all_combinations = Vec::new();
        let mut frontier = Vec::new();

        frontier.push((0, 0, Vec::new()));
        while let Some((sum, candidate_index, mut combination)) = frontier.pop() {
            if sum == target {
                all_combinations.push(combination.to_vec());
                continue;
            }
            if candidate_index == candidates.len() {
                continue;
            }

            let candidate_sum = candidates[candidate_index] + sum;
            if candidate_sum <= target {
                combination.push(candidates[candidate_index]);
                frontier.push((candidate_sum, candidate_index, combination.clone()));
                combination.pop();
            }

            frontier.push((sum, candidate_index + 1, combination));
        }

        all_combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_test() {
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
