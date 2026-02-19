// Step2a
// 目的: step2.rsのコードを改善する

/*
  改善する時に考えたこと
  - 引数6つは多すぎるので減らしたい。
    - sumがtargetに達するか見るのではなく、targetから初めてちょうど0になるかを見るようにすればsumが要らなくなる
    - candidates.split_first()により、candidate_indexの引き継ぎをなくして引数を減らす
      https://github.com/Yoshiki-Iwasa/Arai60/pull/57/changes#diff-d1aabe64d3b8ecf58a421e7472d87dea56ba3ec30721950fd3a93272eed8fa2aR60

  所感
  - candidates.split_first()の部分が自分の考えの外にあったので感心した。
*/

pub struct Solution {}
impl Solution {
    const MIN_CANDIDATE: i32 = 2;

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combination = Vec::new();
        let mut all_combinations = Vec::new();

        Self::make_combinations(&candidates, target, &mut combination, &mut all_combinations);

        all_combinations
    }

    fn make_combinations(
        candidates: &[i32],
        complement: i32,
        combination: &mut Vec<i32>,
        all_combinations: &mut Vec<Vec<i32>>,
    ) {
        if complement == 0 {
            all_combinations.push(combination.to_vec());
            return;
        }
        if complement < Self::MIN_CANDIDATE {
            return;
        }

        let Some((&candidate, rest_candidates)) = candidates.split_first() else {
            return;
        };

        Self::make_combinations(rest_candidates, complement, combination, all_combinations);

        if candidate <= complement {
            combination.push(candidate);
            Self::make_combinations(
                candidates,
                complement - candidate,
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
    fn step2a_test() {
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
