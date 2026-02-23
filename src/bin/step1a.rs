// Step1a
// 目的: 再帰からスタックを利用した実装に書き換える練習

/*
  所感
  - parenthesisをfrontierにpush()するときに毎回clone()しているのが気になったがこの実装では仕方がないと思った。
  再帰処理ではparenthesis.pop()されるのが再帰処理が実行された後だが、この実装ではfrontierにpushした後にすぐparenthesis.pop()が実行されるので厳密に処理の順序が等価ではないことが原因だと思った。
*/

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut all_parenthesis = Vec::new();
        let mut frontier = Vec::new();

        frontier.push((0, 0, String::new()));
        while let Some((open_bracket_count, close_bracket_count, mut parenthesis)) = frontier.pop()
        {
            if open_bracket_count == n && close_bracket_count == n {
                all_parenthesis.push(parenthesis);
                continue;
            }

            if open_bracket_count < n {
                parenthesis.push('(');
                frontier.push((
                    open_bracket_count + 1,
                    close_bracket_count,
                    parenthesis.clone(),
                ));
                parenthesis.pop();
            }

            if close_bracket_count < open_bracket_count {
                parenthesis.push(')');
                frontier.push((
                    open_bracket_count,
                    close_bracket_count + 1,
                    parenthesis.clone(),
                ));
                parenthesis.pop();
            }
        }

        all_parenthesis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1a_test() {
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
