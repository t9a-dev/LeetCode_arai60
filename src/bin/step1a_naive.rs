// Step1a_naive
// 目的:　練習のために別の解法による実装を行う

/*
  問題の理解
  - 整数m,nでが入力として与えられる。m * nで表されるグリッドの左上(0,0)にロボットが配置される。
  このロボットが右下(m-1,n-1)に向かって移動する。ロボットが右下に到達するために取れる一意の経路の総数を返す。
  ロボットは右又は下方向にしか移動できない。
  m=3 n=7 output=28
  m=3 n=2 output=2
  m=2 n=2 output=2

  何がわからなかったか
  - 再帰DFSでの計算量の見積もり。

  何を考えて解いていたか
  - 右に行けるだけ行く、下に行けるだけ行くという方針で再帰DFSを実装してみる。
  基本ケース
    - 右下の座標に到達したときに1つの経路として1を返す。
    - 右に行けなくなったら0を返す。
    - 下に行けなくなったら0を返す。
    戻り値を全て足して返す。
    実装しようとして気付いたが、引いていく（右下から左上のスタートへ）方が実装しやすそうなので、引いていく。
      基本ケース
        - 左上の座標（1,1）に到達したときに1つの経路として1を返す。
      再帰ケース
        - 左(col-1)に行けるとき再帰呼び出し。左(col-1)に行けないときは再帰呼び出ししない。
        - 上(row-1)に行けるとき再帰呼び出し。上(row-1)に行けないときは再帰呼び出ししない。
        戻り値を全て足して返す。
  再帰DFSで計算量を見積もりたいが、考え方がよく分からない。あとでChatGPTに聞く。
  Leet Code採点システムでTime Limit Exceededとなるコードが出来上がった。テストケースはm=23,n=12
  ChatGPT(GPT-5.1)によると時間計算量はO(2 ^ (m+n))とのこと。Time Limit Exceededになったテストケースに当てはめると解は34,359,738,368だった。
  最適化として思いつくのは、HashSetに(row,col)でvisitedに入れていき、すでに訪れた座標は再帰呼び出しせずに0を返す。
  ↑この方法は試して間違っていることに気づいた。HashMapで座標をキーにその経路を通ったときの結果を返すべき。
  HashMapによる最適化でAcceptedになった。
  時間計算量: O(m * n) すぐに思いつかずChatGPT(GPT-5.1)に聞いた。HashMapでキャッシュしたことで定数時間O(1)でキャッシュを利用できるので、グリッドのセルの数を全部計算する感じだと理解。
  空間計算量: O(m * n) 同上。HashMapによるキャッシュでグリッドのセル数となる。

  所感
  - 時間計算量を見積もってからその実装方針で良いのかの判断をするべき（採点システムガチャをしない）だが、余裕がなく実装するので精一杯だった。
  - Time Limited Exceeded になる実装の時間・空間計算量の見積もりがどうやって良いか分からず、ChatGPT(GPT-5.1)に聞いたが長々と数式が出てきて理解するのに時間がかかりそうなのでスキップした。
  - 動的計画法カテゴリの問題を解き始めてからナイーブな実装すら自分で思いついて書けていないものの、解法の理解と理解した解法を元にコードの変形（別の解法）にかかる時間は短くなってきているので多少進歩を感じる。
*/

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            panic!("m and n grater than 0 required.");
        }

        let mut paths_cache: HashMap<(usize, usize), i32> = HashMap::new();
        Self::explore_unique_paths(m as usize, n as usize, &mut paths_cache)
    }

    fn explore_unique_paths(
        rows: usize,
        cols: usize,
        paths_cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if rows == 1 && cols == 1 {
            return 1;
        }

        let to_left_paths = match cols.checked_sub(1) {
            Some(cols) => match paths_cache.get(&(rows, cols)) {
                Some(to_left_cache) => *to_left_cache,
                None => {
                    let to_left = Self::explore_unique_paths(rows, cols, paths_cache);
                    paths_cache.insert((rows, cols), to_left);
                    to_left
                }
            },
            None => 0,
        };

        let to_up_paths = match rows.checked_sub(1) {
            Some(rows) => match paths_cache.get(&(rows, cols)) {
                Some(to_up_cache) => *to_up_cache,
                None => {
                    let to_up = Self::explore_unique_paths(rows, cols, paths_cache);
                    paths_cache.insert((rows, cols), to_up);
                    to_up
                }
            },
            None => 0,
        };

        to_left_paths + to_up_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step2_test() {
        assert_eq!(Solution::unique_paths(4, 3), 10);
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }

    #[test]
    #[should_panic]
    fn step2_panic_test() {
        Solution::unique_paths(0, 1);
        Solution::unique_paths(1, 0);
        Solution::unique_paths(0, 0);
    }
}
