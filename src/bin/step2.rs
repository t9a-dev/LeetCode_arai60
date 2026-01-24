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
  https://github.com/hayashi-ay/leetcode/pull/51/changes
  https://github.com/olsen-blue/Arai60/pull/50/changes
  - 実装に幅があり参考になる。

  https://github.com/Yoshiki-Iwasa/Arai60/pull/43/changes#r1709433297
  - 累積和を使った実装方法。また、累積和を計算する時にstd::iter::successorsを利用するときれいに書けるとある。
  successorsは知らないメソッドだったので勉強になった。
  std::iter::scanでも良いのでは？と思ったが、先頭に0を持つ累積和の配列を生成するためにnums.scan()としても、配列の先頭に0が含まれていないのでnums[0]に0を入れるような操作が必要になる。
  successorsでは初期値を起点として、Noneが返されるまで繰り返し操作を行うので、先頭に0を持つ累積和の配列を作れる。
  本ファイル下部のplaygroundテストに確認用のコードを実装した。

  https://github.com/naoto-iwase/leetcode/pull/50/changes#diff-d7328f247ea703c897516111c872b729bf96f2b2e54ab850566dd2ac46122d94R48
    > 前からの累積和は単調増加するので、二分探索（lower bound）で初めて和がtarget以上になる点がわかる。
  - 累積和と二分探索を組み合わせた解法について、わかりやすく一言でまとまっている。

  改善する時に考えたこと
  - 最小部分配列長さの初期値を表す変数nは雑すぎるので、n -> sentinel_subarray_lengthに変更する。
  max_subarray_lengthにしようかと思ったが、nums.len() + 1は明らかにmax_subarray_lengthではないので、sentinelとした。
  - 変数名sumでも十分な気はするが、丁寧にするなら sum -> prefix_sumとする。減算もするので。
  - 条件を満たす部分配列が見つからなかった時の戻り値0を定数で宣言する。

  所感
  - 実装に幅があるので写経しておく
    - 入力numsの累積和を計算してから利用する方法
      https://github.com/Yoshiki-Iwasa/Arai60/pull/43/changes#diff-705d13bf86ae090a0d8b47599b0a08e57431901d11cce1cdd7c2dea2f2568047R43
      - 二分探索も利用している
        https://github.com/naoto-iwase/leetcode/pull/50/changes#diff-d7328f247ea703c897516111c872b729bf96f2b2e54ab850566dd2ac46122d94R62
        https://github.com/hayashi-ay/leetcode/pull/51/changes#diff-6d4eb2707ed57d8037bfa2e5985424b237a407741a7602ba92b31e090d1cb096R79
*/

pub struct Solution {}
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;

        if nums.is_empty() {
            return NOT_FOUND;
        };

        let sentinel_subarray_length: usize = nums.len() + 1;
        let mut min_subarray_length = sentinel_subarray_length;
        let mut prefix_sum = nums[0];
        let (mut start, mut end) = (0, 0);

        while start <= end && end < nums.len() {
            if target <= prefix_sum {
                min_subarray_length = min_subarray_length.min((end - start) + 1);
                prefix_sum -= nums[start];
                start += 1;
            } else {
                end += 1;
                if end < nums.len() {
                    prefix_sum += nums[end];
                }
            }
        }

        if min_subarray_length == sentinel_subarray_length {
            return NOT_FOUND;
        }

        min_subarray_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let expect = vec![0, 1, 3, 6, 10, 15];
        let nums = vec![1, 2, 3, 4, 5];

        // std::iter::scan
        assert_eq!(
            &[0].iter()
                .chain(nums.iter())
                .scan(0, |state, x| {
                    *state += x;
                    Some(*state)
                })
                .collect::<Vec<_>>(),
            &expect
        );

        // std::iter::successors
        // MEMO: successorsのクロージャ内で nums.iter().map(|num| num + acc) として無限ループになった。
        // nums.iter()で毎回新たなイテレータを生成してしまい.next()が常にnums[0]を返すので無限ループになる。
        // 外側でイテレータを生成して、このイテレータを変更しながら行うべき。
        let mut nums_iter = nums.iter();
        assert_eq!(
            std::iter::successors(Some(0), |acc| nums_iter.next().map(|num| num + acc))
                .collect::<Vec<_>>(),
            expect
        )
    }

    #[test]
    fn step2_test() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }
}
