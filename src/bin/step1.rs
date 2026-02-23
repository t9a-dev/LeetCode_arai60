// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 自然数nが与えられる。n個の括弧の全ての組み合わせを返す。
  n=1のとき["()"] 01(1)
  n=2のとき["(())", "()()"] 0011(3) 0101(5)
  n=3のとき["((()))", "(()())", "(())()", "()(())", "()()()"] 000111(7) 001011(11) 010011(19) 010101(21)

  何を考えて解いていたか
  - 何も思いつかないので、手作業でやることを考える。
    - '('のとき次に来て良いのは'(', ')'
    - ')'のとき次に来て良いのは')', '('
    - n個の'(', ')'の並びから後続の'(', ')'の並びが一意になるかと思ったがそうでもない
  - '(' = 0, ')' = 1としたときのビット列の並びを眺めてみてもとくに何も思いつかない。
  ナイーブな実装も思いつかず手が止まったので実装例を見て理解する。

  何がわからなかったか
  - 有効なペアを維持しながら、全パターンを出力するアルゴリズムが分からなかった。

  実装例の理解
  https://leetcode.com/problems/generate-parentheses/solutions/5976224/complex-backtracking-interview-prepare-l-keak/
  - 組み合わせの文字列に出現する'(', ')'の数は必ずnと等しくなることに注目している。
  - '('が最初にn個連続しても不正なペアは生成されない
    - if open_count < n
      - '(((' -> ')))' = "((()))"
  - ')'は先に出現している'('の個数を超えて連続すると不正なペアが生成される
    - '(' -> '))' -> '(' = "())(" 不正なペアが生成される
    つまり、先に出現している'('の個数を超えない範囲で')'を追加すると不正なペアは生成されない
    - if close_count < open_count
      - '(' -> ')' -> '(' -> ')' = "()()"

  所感
  - 実装例を読めば理解できるものの、自分ではナイーブな実装も思いつかなかったので繰り返し練習が必要だと感じる。
  - とりあえず、スタックの実装に書き直す練習をしておく。(step1a.rs)
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
        open_count: i32,
        close_count: i32,
        parenthesis: &mut String,
        all_parenthesis: &mut Vec<String>,
    ) {
        if open_count == n && close_count == n {
            all_parenthesis.push(parenthesis.clone());
            return;
        }

        if open_count < n {
            parenthesis.push('(');
            Self::make_parenthesis(n, open_count + 1, close_count, parenthesis, all_parenthesis);
            parenthesis.pop();
        }

        if close_count < open_count {
            parenthesis.push(')');
            Self::make_parenthesis(n, open_count, close_count + 1, parenthesis, all_parenthesis);
            parenthesis.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
