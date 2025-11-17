// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数を持つ配列numsが与えられる。numsの並び順を変えずに厳密に増加する部分列を作ったときに最長となる部分列の長さを返す。
  case1: nums=[1,0,2,3] subsequence=[1,2,3] return=3
  case2: nums=[0,1,0,3,2,3] subsequence=[0,1,2,3] return=4
  case3: nums=[3,3,3,3] subsequence=[3] return=1

  何がわからなかったか
  - うまく部分問題に分割しきれなかった。
  Wron Answerとなったテストケースを見ると、subsequenceの末尾の次に追加するべきなのは、候補となる数列を先頭から見ていき、最小をとなる数値で更新する感じだと思うがうまく言語化できない。

  何を考えて解いていたか
  - 問題は理解できたと思うので、自分がテストケースの例をどうやって手作業で解いたのか考えてみる。
    - nums[i]に注目する。nums[i+1]がnums[i]より大きければsubsequenceに追加して注目している部分をnums[i+1]に更新する。
      - この方法だと処理できないケースがあるので、nums[i-1] < nums[i+1] < nums[i] であれば、subsequenc.pop(),subsequence.push(nums[i+1])する。
  - 最後にsubsequenceのうち最大サイズ判定して返す。
  - ナイーブな実装だと全ての組み合わせを確認するのでO(n ^ 2)となるがまずは手作業でやることをコードに落とし込むことを目標とする。

  step1a.rsで解答例の写経と理解を行う
*/

pub struct Solution {}
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        /*
          この実装はWrong Answerとなったものです。

          nums = [5,7,-24,12,13,2,3,12,5,6,35]
          Output = 5
          Expected = 6
        */
        let mut subsequences: Vec<Vec<i32>> = Vec::new();
        for v in nums.iter() {
            subsequences.push(vec![*v]);
        }

        for (num_index, subsequence) in subsequences.iter_mut().enumerate() {
            let mut current = subsequence[0];
            let nums = &nums[num_index + 1..];

            for (i, next) in nums.iter().enumerate() {
                if &current < next {
                    subsequence.push(*next);
                    let prev = current;
                    current = *next;

                    let Some(next) = nums.get(i + 1) else {
                        continue;
                    };

                    if &prev < next && next < &current {
                        subsequence.pop();
                        subsequence.push(*next);
                        current = *next;
                    }
                }
            }
        }

        subsequences
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        // failed left=5,right=6
        // assert_eq!(
        //     Solution::length_of_lis(vec![5, 7, -24, 12, 13, 2, 3, 12, 5, 6, 35]),
        //     6
        // );
    }
}
