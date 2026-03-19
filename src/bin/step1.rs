// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 文字列sと行数を表す整数num_rowsが与えられる。指定された行数に渡ってジグザグに並べ替えた文字列を返す。
  s="PAYPALISHIRING" num_rows=3 out="PAHNAPLSIIGYIR"

  何を考えて解いていたか
  - 紙に書いて規則性を見るのが良さそう。
  - 入出力例は以下のようになっている。
    - num_rows=3のとき、に0列目に3文字・1列目に1文字・2列目に3文字・3列目に1文字...となっている。
    - num_rows=4のとき、0列目に4文字・1列目に1文字・2列目に1文字・3列目に4文字・4列目に1文字・5列目に1文字・6列目に残りの文字...となっている。
    - i + num_rows - 1 列目の列でnum_rows文字出力している。この列以外は1文字だけ出力している
  ここで手が止まったので答えを見る。

  何がわからなかったか
  - 規則性のようなものを見つけたが、手作業でやる方法まではたどり着けなかった。

  解法の理解
  https://www.youtube.com/watch?v=Q2Tw6gcVEwc&t=1s
    - NeetCodeの解説動画を見たがマジックナンバーだらけで分かりづらいと感じる。
  https://neetcode.io/solutions/zigzag-conversion
    - このページに掲載されている「2. Iteration - II」の方が分かりやすそう
  - LeetCode問題文でZigzagになった文字列の並びが示されているとおりに、0行目0列目からスタートしていくイメージだと理解した。
    - 0行目〜(num_rows - 1)行目まで進みながらs[i]の文字を配列にpushしていくと、i行目に対応する文字を配列に格納できる。
    - (num_rows - 1)に到達したら、逆順(num_rows - 1)行目~0行目に進みながら、i行目に対応する文字を配列に格納できる。
      - directionで行の進む方向を制御している。
  - 最終的に["abc","def","ghi"]のような配列が得られるので、concatで1つの文字列にまとめている。

  所感
  - 文字列の並びの規則性を見つけることにばかり気を取られて、手作業で問題文に示されている通りZigzagに文字をなぞっていく選択肢を思いつかなかった。
  問題を解く時にいきなりきれいなアルゴリズムを考えるのではなくて、シンプルに手作業で解くということを実践したいなと思った。
  感覚としてスマートな解法を考えることにばかり気を取られていて、問題に対して素直に考えられていないという感じがする。
*/

pub struct Solution {}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = match num_rows.try_into() {
            Ok(v) => v,
            Err(_) => panic!("num_rows must be positive value. num_rows: {}", num_rows),
        };

        if num_rows == 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row = 0usize;
        let mut direction = 1;

        for c in s.chars() {
            rows[row].push(c);
            row = ((row as i32) + direction) as usize;
            if row == 0 || row == num_rows - 1 {
                direction *= -1;
            }
        }

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn step1_num_rows_negative_value_test() {
        Solution::convert("PAYPALISHIRING".to_string(), -3);
    }

    #[test]
    fn step1_test() {
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
