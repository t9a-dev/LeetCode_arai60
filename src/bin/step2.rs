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
  講師陣はどのようなコメントを残すだろうか？
  Q1. try_into().unwrap()ではなく、as i32によるキャストで十分では？
  A1. as演算子では「結果が範囲外であったり予期しない値である場合でも、変換は問題なく（ただし不適切な）値を生成します」という旨の記述があったので、
  as演算子を利用する明確な理由がなければtry_into()を利用するべきであると判断した。
  https://rust-for-c-programmers.salewskis.de/ch16/16_2_casting_with_as.html#1625-performance-considerations

  他の人のコードを読んで考えたこと
  - ハッシュテーブルを利用した配列を一回だけ走査するワンパスアルゴリズムがあることが分かり、ハッシュテーブルをから値を探すので結局そんなに計算量は変わらないのでは？
  と思ったが、調べたところハッシュテーブルの操作はほぼO(1)で実行でき、最悪の場合でも全体の計算量はO(n^2)になるので、2重ループによる総当りで計算量が常にO(n^2)よりも優れている（殆どのケースで速い）のかと感心した。

  改善する時に考えたこと
  - forで範囲を手動で指定するのではなく、配列のサイズからメソッドで安全に取得したい。
*/

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num_i) in nums.iter().enumerate() {
            //　ペアを見たいので同じインデックスはskip()で飛ばす
            for (j, num_j) in nums.iter().enumerate().skip(i + 1) {
                if (num_i + num_j) == target {
                    return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
