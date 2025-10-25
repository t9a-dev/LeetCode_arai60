// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 初期値で整数の配列nums（テストの点数）、整数k（テストの点数が大きい順の順位）が与えられる。
  add処理では整数（テストの点数）が与えられるので、テストの点数のリストに加えて、k番目(順位)のテストの点数を返す。
  数値が大きいほど点数は高いので、順位は上になる。

  何がわからなかったか
  - 問題自体を理解するのに少し時間がかかった。一回読むだけですぐには理解しきれなかった。
  - 実行速度の遅さから最適化の余地があるが、こういった問題に適用できるような最適化の手段がわからなかった。（現状自分の引き出しにない感じ）

  何を考えて解いていたか
  - addが呼び出されるたびに、
    - 既存の配列に点数を追加
    - 配列で降順ソート実施
    - k番目の位置の値を返す
  という流れで実装する。

  正解してから気づいたこと
  - ナイーブな実装をしたが実行速度が遅いので毎回ソートしている箇所を改善できると思った。
  LeetCode上で760ms前後でBeats6.45%と遅い。
  - 思いつく最適化手段は以下
    - 初期化時にソートして、k番目の値を把握しておく。
    - addで追加された値がk番目の値以下ならソートせずにk番目の値を返す。
    - ソートした場合は返却するk番目の値を更新する。
  この最適化を試してみたいのでstep1a.rsで試してみる
*/

pub struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        // LeetCodeの定義上kをi32型としているが、配列中先頭からのある位置を表すのであればusizeとして、型レベルでkをバリデーションするべきであると考える。
        // kをusizeとした場合で0が渡されたときはpanic!意外に常に1として扱うという方法もあると考えた。
        if k <= 0 {
            panic!("k must be 1 or greater.")
        }
        Self { k: k - 1, nums }
    }

    fn add(&mut self, num: i32) -> i32 {
        self.nums.push(num);
        self.nums.sort_by(|a, b| b.cmp(a));

        match self.nums.get(self.k as usize) {
            Some(v) => *v,
            None => i32::MIN,
        }
    }
}

pub struct Solution {}
impl Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_case1_test() {
        let mut score_manager = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(score_manager.add(3), 4);
        assert_eq!(score_manager.add(5), 5);
        assert_eq!(score_manager.add(10), 5);
        assert_eq!(score_manager.add(9), 8);
        assert_eq!(score_manager.add(4), 8);
    }

    #[test]
    fn step1_case2_test() {
        let mut score_manager = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(score_manager.add(2), 7);
        assert_eq!(score_manager.add(10), 7);
        assert_eq!(score_manager.add(9), 7);
        assert_eq!(score_manager.add(9), 8);
    }
}
