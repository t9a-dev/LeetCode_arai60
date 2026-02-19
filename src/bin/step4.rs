// Step4
// 目的: シグネチャの改善

/*
  入力の制約上はありえないが、candidates[i]に0又は負数が入ると無限ループするのが気になるので、シグネチャを変更して型レベルで無限ループを防止する。

  所感:
  - 呼び出し側（テストコード）が少し煩雑になるものの、combination_sumの引数candidatesのシグネチャがVec<i32>であるにも関わらず、負数が入ると無限ループするよりは良いと思った。
  問題なさそうに動きつつある日突然バグが顕在化するよりは良いという感覚。
  - 引数チェックとしてcandidatesをfileterしても良いが、型で防げるならこちらのほうが計算量的、仕様の明確さが優れているという感覚。
*/

use std::num::NonZeroU32;

pub struct Solution {}
impl Solution {
    /*
      このコードはLeet Code採点システムを通りません。
      関数のシグネチャが採点システムと一致しないためです。
    */
    pub fn combination_sum(candidates: Vec<NonZeroU32>, target: u32) -> Vec<Vec<NonZeroU32>> {
        let mut all_combinations = Vec::new();
        let mut combination = Vec::new();

        Self::make_combinations(
            &candidates,
            0,
            target,
            &mut combination,
            &mut all_combinations,
        );

        all_combinations
    }

    fn make_combinations(
        candidates: &[NonZeroU32],
        sum: u32,
        target_sum: u32,
        combination: &mut Vec<NonZeroU32>,
        all_combinations: &mut Vec<Vec<NonZeroU32>>,
    ) {
        if sum == target_sum {
            all_combinations.push(combination.to_vec());
            return;
        }

        let Some((&candidate, rest_candidates)) = candidates.split_first() else {
            return;
        };
        let candidate_sum = candidate.get() + sum;

        if candidate_sum <= target_sum {
            combination.push(candidate);
            Self::make_combinations(
                candidates,
                candidate_sum,
                target_sum,
                combination,
                all_combinations,
            );
            combination.pop();
        }

        Self::make_combinations(
            rest_candidates,
            sum,
            target_sum,
            combination,
            all_combinations,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn non_zero_u32_vec(values: Vec<u32>) -> Vec<NonZeroU32> {
        values
            .into_iter()
            .map(|v| NonZeroU32::new(v).unwrap())
            .collect()
    }

    #[test]
    fn step4_test() {
        let mut expect = vec![vec![2, 2, 3], vec![7]]
            .into_iter()
            .map(non_zero_u32_vec)
            .collect::<Vec<Vec<NonZeroU32>>>();
        let candidates = non_zero_u32_vec(vec![2, 3, 6, 7]);
        let mut actual = Solution::combination_sum(candidates, 7);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
            .into_iter()
            .map(non_zero_u32_vec)
            .collect::<Vec<Vec<NonZeroU32>>>();
        let candidates = non_zero_u32_vec(vec![2, 3, 5]);
        let mut actual = Solution::combination_sum(candidates, 8);
        expect.iter_mut().for_each(|x| x.sort());
        expect.sort();
        actual.iter_mut().for_each(|x| x.sort());
        actual.sort();
        assert_eq!(expect, actual);

        let expect = Vec::<Vec<u32>>::new()
            .into_iter()
            .map(non_zero_u32_vec)
            .collect::<Vec<Vec<NonZeroU32>>>();
        let candidates = non_zero_u32_vec(vec![2]);
        let actual = Solution::combination_sum(candidates, 1);
        assert_eq!(expect, actual);

        let expect = Vec::<Vec<NonZeroU32>>::from_iter(vec![vec![]]);
        let candidates = non_zero_u32_vec(vec![2]);
        let actual = Solution::combination_sum(candidates, 0);
        assert_eq!(expect, actual);
    }
}
