// Step2a
// 目的: 別の書き方で練習する。

/*
  odaさんのコメント
  https://github.com/olsen-blue/Arai60/pull/42/changes#r1993204654
    > left を「左側、つまり、条件を満たさないことが判明している左側の最大の場所」として書くこともできます。
    > そうすると、初期値は -1 ですね。
    > この right も「条件を満たさないことが判明している右側の場所の最小」と思うと、right = middle が素直に見えるはずです。

  - step2.rsで書こうとしてうまく実装できなかった start = 0, end = nums.len() を初期値として実装する練習を行う。
  step2.rsの実装が自然に感じるが、あえて違和感のある方法で練習することで理解を深める狙い。
  https://github.com/t9a-dev/LeetCode_arai60/pull/41#discussion_r2607036540

  解法の理解
    - 常に配列の最後の要素より小さいかでendを更新している
      - 配列の最後は動かないのになぜこれで良いのだろうか。
        - 配列の最後は配列中の値の最大値にも最小値にもなり得る。
        - 配列外(nums.len())をより大きい値の集合の始まりとして見ている？
          - つまり右側とは何か値の集合があると仮定して、nums.last()より大きい値しか無いという番兵の考え方だと理解した。
          [ nums[i] <= nums.last() | nums.last() < ]
    - 左側により小さい値を見つけたらそこまで探索範囲を縮小している。nums[middle] <= nums.last() then end = middle
      - <= としているのはnums.last()が最小値の可能性があるので。
    - 右側(end)により小さい値があれば、左側を捨てる。 nums.last() < nums[middle] then start = middle + 1 // middleを探索範囲外にする。

  所感
  - nums.last()との比較でかなり面食らう感じがある。nums.last()は最大値、最小値、その他の値のどれでもあるのになぜ比較しているのか？と初見で思った。
  自分には認知負荷が高い。
  (start,end]な半開区間も実装しようかと思ったが時間切れなのでスキップ。
*/

pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty");
        }

        // [start,end)
        // start <= i < end
        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let middle = start + (end - start) / 2;

            if nums[middle] <= *nums.last().unwrap() {
                end = middle;
            } else {
                start = middle + 1;
            }
        }

        nums[start]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
        assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![2, 3, 1]), 1);

        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn step1_empty_nums_test() {
        let empty_nums = Vec::new();
        Solution::find_min(empty_nums);
    }
}
