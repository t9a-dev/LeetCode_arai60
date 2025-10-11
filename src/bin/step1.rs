// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解:
  メールアドレス文字列の配列が与えられる。メールを送信すべき重複のないメールアドレス文字列の数を返す。
  メールアドレスはローカル名部分とドメイン部分で構成されており、@以前がローカル名、@以降がドメイン名となっている。
  メールアドレスのローカル名部分について+以降の文字列は無視されローカル名として扱われない。
  メールアドレスのローカル名部分に.が含まれる場合.は無視される

  何がわからなかったか
  - @でメールアドレスを分割したあとにローカル部分、ドメイン部分のタプルに分割する書き方がわからず詰まった。
  - whle let構文の存在を忘れていた。

  何を考えて解いていたか
  - メールアドレスのローカル部分とドメイン部分の分割はsplit系のメソッドでできそう
  - ローカル部分の文字列を全操作して.をスキップ、+が出現したら即リターンでローカル名を正規化できそう
  - メールアドレスの数を返せばよいのでHashSetに追加していって最後に数をカウントして返す
  - 正規表現で実現できそうだが、具体的なパターン記述が思い浮かばない
  - ルールに従わないメール文字列は即リターンでスキップする

  正解してから気づいたこと
  - 変数名についてlocal_iterよりlocal_charsの方がわかりやすそう
  - splited_emailの方はVec<_>にしてコンパイラが型推論できるところは型推論させた方がよさそう
  - local,domainをタプルで取り出す際にパターンマッチングしなかったときにpanicはやりすぎな気がする。
  continueにして無視する方が現実的。loggerが使えればwarnレベルで不正なメールアドレスを記録しておくのが現実的だと考えた。
  - ドメイン部分が.comで終わることの検証が抜けている。今回のコードでは@が含まれていて、分割後のパーツ数が2であることを見ているので、
  検証するのが自然だと思った。
  - 変数名についてunique_send_emailsではなく、unique_emailsの方が余計な情報がなくて良いと思った。
  - splitでローカル名部分とドメイン部分を分割している処理をなくせそうだと思った。
  - splitをなくす場合は@を見つけたか、+を見つけたかどうかを判定するミュータブルなフラグを用意しておき見つけ次第フラグを立てて+から@までの文字列、ドメイン部分を判定する。
  この変更でsplitによる全走査が減らせる。フラグが立っていなければ不正なメールアドレスとしてスキップしたり、例外処理できる。
  コードが複雑になるので、Runtime 0ms出ている現状は複雑になる方向に修正する意味は感じられない。

  memo:
  解法は思いついたものの、文字列操作やタプルへの取り出しなどをどのように書けば良いのか調べていたら1時間ほどかかってしまった。
*/

use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let valid_email_parts_count = 2;
        let mut unique_send_emails: HashSet<String> = HashSet::with_capacity(emails.len());

        for email in emails.into_iter() {
            let splited_email: Vec<&str> = email.split('@').collect();

            if valid_email_parts_count != splited_email.len() {
                continue;
            }

            let (local, domain) = match splited_email[..] {
                [local, domain] => (local, domain),
                _ => panic!("failed parse email address: {}", &email),
            };
            let mut sanitized_local = String::new();
            let mut local_iter = local.chars().into_iter();
            while let Some(c) = local_iter.next() {
                match c {
                    '+' => break,
                    '.' => (),
                    _ => sanitized_local.push(c),
                }
            }

            unique_send_emails.insert(sanitized_local + "@" + domain);
        }

        unique_send_emails.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn step1_test() {
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
}
