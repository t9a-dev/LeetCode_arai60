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
  - 流石にここの文章を読んだだけでは理解できなかったが、手元で処理の流れをトレースしていったら図の意味が分かった。
  https://github.com/olsen-blue/Arai60/pull/31/files#diff-b7fbb0dce1473afc0264185268f1a1ef6d682a3a8c997d43bc8bdd636a66ce4aR35-R46

  - Rust実装がなのでロジックに集中して読めて理解の助けになった。
  https://github.com/Yoshiki-Iwasa/Arai60/pull/46/files

  - 「セグメント木（セグメントツリー）」は初めて聞いた。常識に含まれていないものの、複雑なデータ構造の代用品として重宝されるそう。
  ここまで時間使いすぎたのでメモのみ。
  https://github.com/docto-rin/leetcode/pull/36#discussion_r2468854052

  - いきなりO(n log n)の解法を思いつけなくてもO(n ^ 2)の解法を実装できていて良いなと思った。
  問題を正しく理解して、とりあえずナイーブな実装だけでもできるようになりたいなと思う。
  https://github.com/docto-rin/leetcode/pull/36/files

  - 3週間前に自分がアホなコメントした問題だった。全然覚えてなかった。
  https://github.com/docto-rin/leetcode/pull/36/files#r2464866724
  自分のstep1b.rsとロジックは同じだなと思った。
  bisect_leftもstep1b.rsにおけるpartition_pointと同じインデックスを返してくるので、min_tailsにpushするかの条件判定を変えれば+1,-1しなくて良くなるのかと今になって理解できた。
  https://docs.python.org/ja/3.13/library/bisect.html#bisect.bisect_left
  ```python3
  from typing import List
  from bisect import bisect_left

  def lengthOfLIS(self, nums: List[int]) -> int:
    min_tails = []
    for num in nums:
        next_length = bisect_left(min_tails, num)
        if next_length >= len(min_tails):
            min_tails.append(num)
            continue
        min_tails[next_length] = num
    return len(min_tails)
  ```

  改善する時に考えたこと
  - lisが気になったが、特に良い変数名も思いつかないのでコメントで対応する。

  所感
  - 一度理解すればこの実装が素直に思える。
  - なぜこれが正しいのかを自分で一度試したので、暗記でロジックを書いている感じ。
    数週間後にやり直しても解けるかも知れないが、もう一度この手順で動くのか確認する作業が必要になりそうだなと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        // lis(longest_increasing_subsequence)
        let mut lis = Vec::new();

        for num in nums {
            let insert_position = lis.partition_point(|num_in_lis| *num_in_lis < num);

            if lis.len() <= insert_position {
                lis.push(num);
                continue;
            }

            lis[insert_position] = num;
        }

        lis.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_ground() {
        let insert_position = vec![0].partition_point(|v| *v < 0);
        assert_eq!(insert_position, 0);
        let insert_position = vec![0].partition_point(|v| *v < 1);
        assert_eq!(insert_position, 1);

        let insert_position = vec![0].partition_point(|v| *v < -1);
        assert_eq!(insert_position, 0);
    }

    #[test]
    fn step2_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(
            Solution::length_of_lis(vec![5, 7, -24, 12, 13, 2, 3, 12, 5, 6, 35]),
            6
        );
        assert_eq!(Solution::length_of_lis(vec![1, 98, 99, 100, 2, 4]), 4);
        assert_eq!(Solution::length_of_lis(vec![7]), 1);
        assert_eq!(Solution::length_of_lis(vec![2, 1]), 1);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
    }
}
