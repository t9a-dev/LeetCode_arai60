// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = prices.len()
  時間計算量: O(n)
  空間計算量: O(1)
*/

/*
  1回目: 1分24秒
  2回目: 1分38秒
  3回目: 1分14秒
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = match prices.len() {
            0 => return 0,
            _ => prices[0],
        };
        let mut max_price = 0;

        for price in &prices[1..] {
            min_price = min_price.min(*price);
            max_price = max_price.max(*price - min_price);
        }

        max_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
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
