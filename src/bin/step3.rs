// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

use std::collections::HashSet;

/*
  入力emailsのサイズをN、emails[i]の平均文字列長をLとする。
  時間計算量: O(N * L)
  空間計算量: O(N * L)
*/

/*
  1回目: 8分13秒
  2回目: 7分3秒
  3回目: 6分27秒
*/
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
    fn step3_test() {
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
    fn canonicalize_sendable_email_address_test() {
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
        assert_eq!(
            Solution::canonicalize_email_address("abc.+abc@abc.com"),
            Some("abc@abc.com".to_string())
        );
        assert_eq!(
            Solution::canonicalize_email_address(".a+l.e.e.t+.code@leetcode.com"),
            Some("a@leetcode.com".to_string())
        );
        assert_eq!(Solution::canonicalize_email_address("leetcode.com"), None);
        assert_eq!(Solution::canonicalize_email_address("leetcode"), None);
        assert_eq!(
            Solution::canonicalize_email_address("@leet@code@.com"),
            None
        );
        assert_eq!(
            Solution::canonicalize_email_address("@leet@code@.com"),
            None
        );
        assert_eq!(
            Solution::canonicalize_email_address(".+l.e.e.t+.code@leetcode.com"),
            None
        );
    }
}
