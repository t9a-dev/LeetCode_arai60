// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 配列の中から2つの数値を選んでtargetになる数値のnumsのindexを返すという理解ができたが、
  5分以内にプログラムに落とし込むことができず手が止まった。

  何を考えて解いていたか
  - 配列の先頭から全ての値のペアを確認していってtargetと等しくなるペアを確認しようと考えていた。
  - 答えに対して必ず1つの解があるという前提なので、メソッドの戻り値の型に合わせて空の配列を返せば良いかと考えた。

  正解してから気づいたこと
  - 答えを見てすぐに何をしているのかは理解できたので、アルゴリズム系のコードを見てこなかった、書いてこなかったことが原因で、
  手が止まってしまったのかという気付きがあった。
  - 配列を全て見ていくことが決まっているので、0..nums.len()以外に良い書き方がありそう
  - 配列から任意の位置の値を取り出すとき、for文で配列の範囲を自身で指定しなければ、indexは安全に取得できるので、
  unwrap()を使わずに実装できないか。
*/

pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            let num_i = nums.get(i).unwrap();
            for j in (i + 1)..nums.len() {
                let num_j = nums.get(j).unwrap();

                if (num_i + num_j) == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);

        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);

        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
