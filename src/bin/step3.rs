// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量:
  空間計算量:
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
