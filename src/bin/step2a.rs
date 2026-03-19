// Step2a
// 目的: 別の解法の写経と理解

/*
  https://leetcode.com/problems/zigzag-conversion/solutions/333761/rust-0ms-4ms-by-obliquemotion-ceg6/
  LeetCode Solutionのトップにあった解法。読みづらいので練習としては良さそう。

  解法の理解
  入力例: s="PAYPALISHIRING" num_rows=3 out="PAHNAPLSIIGYIR"
  - (0..num_rows).chain((1..num_rows - 1).rev())
    - [0,1,2,1]
  - cycleで[0,1,2,1]最後尾の要素まで到達すると先頭の要素に戻るイテレータを作っている
  - zipでcycleのイテレータと文字のタプルのセットを生成している
        zip(s.chars()); (row_index,c) -> (0,P)
        zip(s.chars()); (row_index,c) -> (1,A)
        zip(s.chars()); (row_index,c) -> (2,Y)
        zip(s.chars()); (row_index,c) -> (1,P)
        zip(s.chars()); (row_index,c) -> (0,A)
        zip(s.chars()); (row_index,c) -> (1,L)
        zip(s.chars()); (row_index,c) -> (2,I)
        zip(s.chars()); (row_index,c) -> (1,S)
        zip(s.chars()); (row_index,c) -> (0,H)
        zip(s.chars()); (row_index,c) -> (1,I)
        zip(s.chars()); (row_index,c) -> (2,R)
        zip(s.chars()); (row_index,c) -> (1,I)
        zip(s.chars()); (row_index,c) -> (0,N)
        zip(s.chars()); (row_index,c) -> (1,G)
   - zigzags.sort_by_keyの行位置のみでソートしている
        zigzags.sort_by_key(); (row_index,c) -> (0,P)
        zigzags.sort_by_key(); (row_index,c) -> (0,A)
        zigzags.sort_by_key(); (row_index,c) -> (0,H)
        zigzags.sort_by_key(); (row_index,c) -> (0,N)
        zigzags.sort_by_key(); (row_index,c) -> (1,A)
        zigzags.sort_by_key(); (row_index,c) -> (1,P)
        zigzags.sort_by_key(); (row_index,c) -> (1,L)
        zigzags.sort_by_key(); (row_index,c) -> (1,S)
        zigzags.sort_by_key(); (row_index,c) -> (1,I)
        zigzags.sort_by_key(); (row_index,c) -> (1,I)
        zigzags.sort_by_key(); (row_index,c) -> (1,G)
        zigzags.sort_by_key(); (row_index,c) -> (2,Y)
        zigzags.sort_by_key(); (row_index,c) -> (2,I)
        zigzags.sort_by_key(); (row_index,c) -> (2,R)
        - ソートする時にタプルで指定すると、同じ行でアルファベットで並び替えされておかしな結果になるので、sort_by_keyによって明示的に行位置のみを対象にしてソートしている

  所感
  - cycle(),zip()メソッドの動き方を理解するのに時間がかかった。
  関数型の考え方？に慣れていないせいか、cycle()で生成したイテレータをzip()メソッドで文字列の各文字とタプルにしている部分が難しく感じた。inspect()を利用して何が行われているかを理解できた。
  https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.inspect

  - https://github.com/olsen-blue/Arai60/pull/61#discussion_r2040670667
  Pythonだが、近い発想で書かれているコードだなと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            // 複数のイテレータを連結しているコードのデバッグ手法の例として意図的にinspect()のコードを残しています
            .inspect(|(row_index, c)| {
                println!("zip(s.chars()); (row_index,c) -> ({},{})", row_index, c)
            })
            .collect::<Vec<_>>();
        zigzags.sort_by_key(|(row_index, _)| *row_index);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );

        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 1),
            "PAYPALISHIRING"
        );

        assert_eq!(Solution::convert("P".to_string(), 3), "P");
    }
}
