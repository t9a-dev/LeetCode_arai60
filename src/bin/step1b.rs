// Step1b
// 目的: 動的計画法の問題に慣れるための練習

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 株価を表す配列pricesが与えられる。prices[i]はある日の株価である。
  - ある日に株を買った場合に、最大になる利益を値として返す。
  - 利益が発生しない場合は0を返す。
  e.g.
  prices = [1, 2, 3, 4] out = (prices[3] - prices[0]) = 3
  prices = [4, 3, 2, 1] out = 0 // どの日に株を購入しても利益が発生しない

  方針
  具体例の内容はHouse Robberだが、動的計画法(Dynamic Programming)自体に広く適用して解法を思いつくまでの過程を練習する。
  https://leetcode.com/problems/house-robber/solutions/156523/from-good-to-great-how-to-approach-most-of-dp-problems/

  何を考えて解いていたか
  - 再帰処理にする場合を考える
    解法を思いつかなかったのでChatGPT(GPT-5.1)の学習サポートモードで質問しながら理解を進めた。
    ある時点iで
    a) 株を売って利益を求める(株価の最小値を知っている必要がある。)
    b) 株を売らない
    といった選択肢が取れる。
    - 基本ケース
      - 株をそれ以上取引できない prices.len() <= i のとき0を返す。取引できないので利益は0
    - 再帰ケース
      - 株を売る場合
        prices[i] - min_price により利益を求める
      - 株を売らない場合
        現在iをi+1でスキップして再帰に入る
      株価の最小値を更新していく必要がある。

  - どの部分をメモ化するか
    - 目的として再帰呼び出しを減らしたい
      iは単調増加するのでprices.len()回までのスタック深さとなる。
      pricesの最大サイズは入力の制約から 10 ^ 5 となる。
      1スタックフレームを50byteと見積もったとき、50byte * (10 ^ 5) = 5,000,000 となり、5,000,000 / (1024 * 1024) = 約5MBとなる
      手元の実行環境で limit コマンドを実行したところスタックサイズ7MBとなったので大分ギリギリな感じがする。LeetCodeの採点システムのスタックサイズはよくわからないがstack overflowするだろうという感覚。
    - iが単調増加するのでメモ化する部分がなく、入力のサイズから再帰による解法自体が筋が悪そう。
    一応スタックオーバーフローするという予想でLeetCode採点システムで実行してみる。
    スタックオーバーフローしなかった。空間計算量の見積もりに間違いがありそうなのでChatGPT(GPT-5.1)に聞く。
      - スタック深さ(再帰呼び出しの回数)の見積もりはおｋ
      - 1スタックフレームの見積もり50byteが悲観的過ぎる
        - make_max_profitの引数のサイズ見積もり　合計28byte
          - prices: &[i32] 16byte(fat pointerと呼ばれるものらしい。) https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
          - i: usize プラットフォーム依存だが、8byteと見積もって良いと考える。64bitプラットフォームが一般的だという仮定。
          - min_price: i32 4byte
        - make_max_profitメソッド内 合計12byte
          - price: &i32 8byte
          - max_profit: i32 4byte
      コンパイラによる最適化などを考慮せず、そのまま見積もると1スタックフレームあたり40byteとなる
      40byte * (10 ^ 5) / (1024 * 1024) = 約3.8MBとなる
      ここから更にコンパイラによる最適化で減る方向に値が動くことを考えるとスタックオーバーフローは大丈夫そうという見積もりになる。
      ただ、入力の制約が動いたらスタックオーバーフローでプログラムがクラッシュすることを考えると、スタック深さのサイズ見積もりから再帰処理による実装は筋が悪そうだと感じる感覚は間違っていないと思った。

  正解してから気づいたこと
  - 入力の制約から最悪空間計算量で利用するスタックサイズが4MBあたりをうろつく時点で再帰処理による実装は忌避感を感じると思った。
  入力として与えられる株価データは実務寄りで考えた場合、データの性質として増えていくものなので、再帰で実装するといずれスタックオーバーフローするだろうという感覚。

  所感
  - fat pointerという概念を知った。
  https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
  - だいぶ脱線した気がするが、空間計算量見積もりの良い練習になった
  - 動的計画法の実装練習で再帰から初めてメモ化して〜という用にメモ化による最適化ができる前提で進めていた。
  当たり前だがメモ化による最適化が適用できないケースが存在するので、入力データに比例してスタックサイズが大きくなるケースでは再帰による実装は筋が悪いなと思った。

  スライスについて
  - スライスを雰囲気で利用していることに気付いた。
  https://doc.rust-jp.rs/rust-by-example-ja/primitives/array.html

  プログラミングRust 第2版(https://www.oreilly.co.jp/books/9784873119786/)より引用
    スライスは厳密には[T]型のデータを指すものであるが、スライスはほとんど常に参照として扱うので&[T]をスライスと呼ぶことがある。
        - プログラミングRust 第2版 P.63 より
    スライス[T]は任意の長さであり得るので、直接変数に格納したり関数の引数として渡すことができない。常に参照として渡される。
        - プログラミングRust 第2版 P.61 より

  スライス: &[i32] 連続するデータの先頭データへのポインタとスライスの長さ（コンパイル時に決定されない）を持つ参照 所有権を持たない。
    スライスは２ワード(連続するデータ先頭データへのポインタ、スライスの長さ)からなる参照なので、fat pointer(太いポインタ)と呼ばれる
    通常のポインタ(thin(細い) pointer)がメモリアドレスだけを持つのに対して、データへのポインタとスライスの長さ２ワード分を持つ参照なのでfat pointer(太いポインタ)と呼ばれる
  Vec<T>: データ構造としてはヒープ領域に連続で配置したデータの先頭アドレスへのポインタ、データのサイズ、キャパシティをもつので、fat pointerか？と思ったが、Vec<T>自体は参照ではなく構造体なのでそもそもポインタではなかった。
  &Vec<T>: Vec<T>への参照なのでthin pointer
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        Self::make_max_profit(&prices, 0, prices[0])
    }

    fn make_max_profit(prices: &[i32], i: usize, min_price: i32) -> i32 {
        let Some(price) = prices.get(i) else { return 0 };

        if *price < min_price {
            return Self::make_max_profit(prices, i + 1, *price);
        }

        let max_profit = Self::make_max_profit(prices, i + 1, min_price).max(*price - min_price);

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn playground() {
        // https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
        // 64bitプラットフォームで実行することを前提としている。仮に32bitプラットフォームで実行すると8byteになるはず。
        // pointerのサイズが64bitの場合にのみテストを実行するように構成できる。面白い。
        // https://doc.rust-lang.org/reference/conditional-compilation.html#r-cfg.target_pointer_width
        assert_eq!(16, std::mem::size_of::<&[i32]>());
    }

    #[test]
    fn step1b_test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 2, 5, 3, 6, 1]), 4);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::max_profit(vec![4, 3, 2, 1]), 0);

        assert_eq!(Solution::max_profit(vec![2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0); // 株を購入できないので利益は0で意味が通ると思う
    }
}
