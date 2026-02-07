// Step2b
// 目的: bit全探索による解法を写経しておく

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  参考にした解法
  https://github.com/Yoshiki-Iwasa/Arai60/pull/56/changes#diff-dfeb6d1f9eadde6cdcec6d3be9247ad90686a5123bf798a79771a6a8f775fb35R21
  - 1<<nums.len()ではnums.len()個の部分集合の数を表している
  - 0u32..(1 << nums.len())では部分集合の数ループを回すことを行っている
  - subset_bitsでは部分集合にnums[i]が含まれているかをbit列で表している。001であれば0番目のbitのみが立っているのでnums[0]が部分集合に含まれる。
  - subset_bits & (1 << i) によりsubset_bitsのi番目のbitが立っているかを判定している
    - subset_bitsのi番目のbitが立っている時、nums[i]が部分集合に含まれているのでSome(nums[i])
    - subset_bitsのi番目のbitが立っていない時、部分集合には含まれていないのでNone

  所感
  - 1 << nums.len() と (subset_bits &　(1 << i))は暗記するようなものに感じた。
    - N個の集合から作れる部分集合の総数は 1 << N で求められる。
    - i番目のbitが立っているかを調べたいときは、対象のbit列 & (1 << i) で求められる。
    - N個の集合から作れる部分集合のうち、i個目の部分集合に値が含まれるかどうかもbit列に情報として含まれている。
      - n(b)のときnは10進数、bは2進数を表す。
      - nums=[1,2,3]のとき、部分集合の総数は 1 << 3(011) = 7個。
        - 1個目の部分集合には、1(001)から0番目のbitが立っているのでnums[0]に対応する1が部分集合に含まれることが分かる。
        - 2個目の部分集合には、2(010)から1番目のbitが立っているのでnums[1]に対応する2が部分集合に含まれることが分かる。
            []        0(000)
            [1]       1(001)
            [2]       2(010)
            [1, 2]    3(011)
            [3]       4(100)
            [1, 3]    5(101)
            [2, 3]    6(110)
            [1, 2, 3] 7(111)
  - しばらく使わないと忘れると思った。こういう解法があるというインデックスが頭に残れば良い方かなという感じ。
*/

pub struct Solution {}
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0..(1 << nums.len()))
            .map(|subset_bits| {
                (0..nums.len())
                    .flat_map(|i| match (subset_bits & (1 << i)) != 0 {
                        true => Some(nums[i]),
                        false => None,
                    })
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2b_test() {
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
