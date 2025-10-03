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
  -

  他の人のコードを読んで考えたこと
  -

  他の想定ユースケース
  -

  改善する時に考えたこと
  -
*/

pub struct Solution {}
impl Solution {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
}

fn main() {
    Solution::add(1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::add(2, 2);
        assert_eq!(result, 4);
    }
}
