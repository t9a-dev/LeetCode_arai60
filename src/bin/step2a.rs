// Step2a
// 目的: 他の人のコードを写経する

/*
  https://github.com/olsen-blue/Arai60/pull/54#discussion_r2022014586
  - ここのやり取りで言われている「(A)B」と分ける考え方がよく分からないので例示されているコードを写経する。
    > https://github.com/olsen-blue/Arai60/pull/54#discussion_r2027288220

  解法の理解
  - 0 < n のとき"()"は解の中に必ず存在する。
  - n == 1 をbase_caseとして"()"を返している部分が a に対応しているように見える。
  - n - 1 - i としている箇所は n - 1個の"()"が b に対応しているように見える。

  所感
  - この解法にたどり着くまでの道のりが理解出来ない。突飛すぎると感じる。
*/

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_string()];
        }

        let mut all_parenthesis = Vec::new();

        for i in 0..n {
            for a in Self::generate_parenthesis(i) {
                for b in Self::generate_parenthesis(n - 1 - i) {
                    all_parenthesis.push(format!("({}){}", a, b));
                }
            }
        }

        all_parenthesis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2a_test() {
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
