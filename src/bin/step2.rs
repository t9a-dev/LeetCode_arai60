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
  - 自分のコードでは.と+を取り除くことをsanitizeと表現していたが、canonicalの方が意味としてあっていると思った。
  メールアドレスの正規化処理について、関数に切り出しているのがテストもしやすいので良いと思った。
  https://github.com/kt-from-j/leetcode/pull/11

  - コメント集でメールアドレスのRFCについて言及されていたので見てみた。NandToTetrisで作った構文解析器の仕様の記述に似ているなと思った。
  コメント集からリンクを辿っていってメールアドレス仕様RFCのページに到達できたが、自分でこの定義部分を短時間で見つけるのは難しいなと思った。
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0#heading=h.n8uegvb8tidr

  - 個人的にはこの実装方針が入力に対してどのような操作をしているかが明確で一番わかり易いと思った。
  https://github.com/skypenguins/coding-practice/pull/24/files#diff-582300f2f8923de23d876e3fce724c4504135135c60d74ed1828d47534b52547R130-R135

  - 正規化処理について関数型スタイルの書き方もあったが、エラーハンドリングで即リターンできる現状の書き方のほうが好みだと思った。
  関数型スタイルで書くかはコーディング規則があればそれに従う、既存コードの書き方に合わせるなどという判断基準で良いと思った。
  https://leetcode.com/problems/unique-email-addresses/solutions/2874912/rust-a-fast-functional-solution/

  改善する時に考えたこと
  - step1で書いたステートマシンではない方法で書くことで、入力の文字列に対して何をしているのかが明確にわかるようにする。
  - メールアドレスの正規化を関数に切り出して、テスタブルなコードにする。
  - メールアドレス正規化処理の戻り値をOption型で表現して、正規化できたメールアドレスのみSomeで返すことを明確にする。
  - 変数名が分かりやすい命名になっているか
  - 入力の制約を満たしているか有効性チェックを行う
*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .filter_map(|email_address| Self::canonicalize_email_address(&email_address))
            .collect::<HashSet<_>>()
            .len()
            .try_into()
            .unwrap()
    }

    fn canonicalize_email_address(email_address: &str) -> Option<String> {
        let [local_part, domain_part] = email_address.split('@').collect::<Vec<_>>()[..] else {
            return None;
        };

        if domain_part.strip_suffix(".com").is_none() {
            return None;
        }

        let canonicalized_local_part = local_part
            .chars()
            .take_while(|&c| c != '+')
            .filter(|&c| c != '.')
            .collect::<String>();

        if canonicalized_local_part.is_empty() {
            return None;
        }

        Some(format!("{canonicalized_local_part}@{domain_part}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        let result = Solution::num_unique_emails(
            vec![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com",
            ]
            .iter()
            .map(ToString::to_string)
            .collect(),
        );
        assert_eq!(result, 2);

        let result = Solution::num_unique_emails(
            vec!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]
                .iter()
                .map(ToString::to_string)
                .collect(),
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn step2_canonicalize_sendable_email_address_test() {
        assert_eq!(
            Solution::canonicalize_email_address("asdf.+asdf@leetcode.com"),
            Some("asdf@leetcode.com".to_string())
        );
        assert_eq!(
            Solution::canonicalize_email_address("abc+.abc@leetcode.com"),
            Some("abc@leetcode.com".to_string())
        );
        assert_eq!(
            Solution::canonicalize_email_address(".a+bc@leetcode.com"),
            Some("a@leetcode.com".to_string())
        );

        assert_eq!(Solution::canonicalize_email_address("leetcode.com"), None);
        assert_eq!(Solution::canonicalize_email_address("leetcode"), None);
        assert_eq!(
            Solution::canonicalize_email_address("@leet@code@.com"),
            None
        );
        assert_eq!(
            Solution::canonicalize_email_address("+abc@leetcode.com"),
            None
        );
        assert_eq!(Solution::canonicalize_email_address("+@"), None);
        assert_eq!(Solution::canonicalize_email_address(".+@"), None);
    }
}
