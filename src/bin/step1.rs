// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 整数m,nでが入力として与えられる。m * nで表されるグリッドの左上(0,0)にロボットが配置される。
  このロボットが右下(m-1,n-1)に向かって移動する。ロボットが右下に到達するために取れる一意の経路の総数を返す。
  ロボットは右又は下方向にしか移動できない。
  m=3 n=7 output=28
  m=3 n=2 output=2
  m=2 n=2 output=2

  何がわからなかったか
  - 解法自体を思いつかなかった。
    具体的には、
    - 経路を数える方法
    - 全ての経路を通る方法

  何を考えて解いていたか
  - 3 * 7は複雑すぎて最初に考えたくないので、3 * 2 のケースから考えてみる。
  右に行けるだけ行く。下に行けるだけ行く。これで1経路は表せる。
  下に行けるだけ行く。右に行けるだけ行く。これでもう1経路も表せる。
  一意の経路を管理するためにHashSetを利用しようかと思ったが、辿った経路のパターンを管理するのは複雑度が上がり筋が悪そう。
  ある座標から、右に行けるなら右に行く、下に行けるなら下に行くといった感じて再帰で表せそうだが、経路のカウントをどうすればよいのかは思いつかない。
  時間切れなので答えを見る。

  解答の理解
  まずいくつかコードを見たが何をしているのか全く分からなかったので、NeetCodeの解説動画を見た。
  https://www.youtube.com/watch?v=IlEsdxuD4lY
  - 右下のゴール地点が持つゴールへの経路の数を1とする。
  ある地点からゴールへの経路の数を右地点が持つ経路の数＋下地点が持つ経路の数として表している。
  (なぜこうなるのかを考えることは数学の証明問題に立ち入りそうなのでとりあえず考えない。)
  解説動画の中の内側のループで
  ```python3
  for j in range(n - 2, -1, -1):
  ```
  としているが、自分のコードでは-1とした。元のコードではrangeの第2引数に-1としており、n - 2から-1まで-1ずつ進むという動きだと理解した。
  https://docs.python.org/ja/3.14/library/stdtypes.html#ranges
  解説動画を見ると、グリッドの右端列と一番下の行は常に1となるので計算をスキップするために範囲外としているので、n - 1とする方が自然言語での説明に合っていると感じた。

  正解してから気づいたこと
  - 解説動画のコードだと読みながら今何をしていて、これから何をするのか理解するのに苦労した。
    - 読みながらi,jは列と行のどっちのことを言っているんだっけ？といった感じで頭のワーキングメモリからすぐに溢れる感じだった。
  - なんの問題でやったかは忘れたが、m,nの内小さい方を外側のループとした方が時間計算量が改善しそうだなと思った。
    - 100行 * 1列の場合
      - 外側のループに行数を指定すると100回ループを実行するが、列数を指定すればループに入らずに済むので時間・空間計算量を改善できると思った。
      ループに入るたびにvec![]で可変長配列を作成しているため。
  - 右に行けるだけ行く、下に行けるだけ行くという方針でDFSによるナイーブな解法も練習のために実装してみる。
*/

pub struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            panic!("m and n grater than 0 required.");
        }

        let (cols, rows) = (m as usize, n as usize);
        let mut tail_row = vec![1; cols];

        for _ in 0..rows - 1 {
            let mut calculate_row = vec![1; cols];

            for col in (0..cols - 1).rev() {
                calculate_row[col] = calculate_row[col + 1] + tail_row[col];
            }

            tail_row = calculate_row;
        }

        tail_row[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::unique_paths(4, 3), 10);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    #[should_panic]
    fn step1_panic_test() {
        Solution::unique_paths(0, 1);
        Solution::unique_paths(1, 0);
        Solution::unique_paths(0, 0);
    }
}
