// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N = n
  時間計算量: O(4 ^ N / sqrt(N))
  空間計算量: O(4 ^ N / sqrt(N))
  他の人のコードを読んでいる時に計算量がカタラン数と呼ばれるものになると知ったが、自分で見積もることはできなかった。
  ソフトウェアエンジニアの常識には含まれていないとのこと。
  - https://github.com/Ryotaro25/leetcode_first60/pull/58#discussion_r1997665903

  - 電気通信大学「離散数理工学」のスライド
  http://dopal.cs.uec.ac.jp/okamotoy/lect/2024/dme/dme2024lect05.pdf
*/

/*
  1回目: 3分47秒
  繰り返し書いても得るものが無さそうなので2,3回目はスキップ
*/

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut all_parenthesis = Vec::new();
        let mut parenthesis = String::new();

        Self::make_parenthesis(n, 0, 0, &mut parenthesis, &mut all_parenthesis);

        all_parenthesis
    }

    fn make_parenthesis(
        n: i32,
        open_brackets_count: i32,
        close_brackets_count: i32,
        parenthesis: &mut String,
        all_parenthesis: &mut Vec<String>,
    ) {
        if open_brackets_count == n && close_brackets_count == n {
            all_parenthesis.push(parenthesis.clone());
            return;
        }

        if open_brackets_count < n {
            parenthesis.push('(');
            Self::make_parenthesis(
                n,
                open_brackets_count + 1,
                close_brackets_count,
                parenthesis,
                all_parenthesis,
            );
            parenthesis.pop();
        }

        if close_brackets_count < open_brackets_count {
            parenthesis.push(')');
            Self::make_parenthesis(
                n,
                open_brackets_count,
                close_brackets_count + 1,
                parenthesis,
                all_parenthesis,
            );
            parenthesis.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        let mut expect = vec!["()"];
        let mut actual = Solution::generate_parenthesis(1);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec!["(())", "()()"];
        let mut actual = Solution::generate_parenthesis(2);
        expect.sort();
        actual.sort();
        assert_eq!(expect, actual);

        let mut expect = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        let mut actual = Solution::generate_parenthesis(3);
        expect.sort();
        actual.sort();
        assert_eq!(actual, expect);
    }
}
