// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  -

  想定ユースケース
  -

  正解してから気づいたこと
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
