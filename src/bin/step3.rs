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
  1回目: 1分42秒
  2回目: 1分46秒
  3回目: 1分40秒
*/

pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let Some((mut holding_price, remaining_prices)) = prices.split_first() else {
            return 0;
        };
        let mut total_profit = 0;

        for price in remaining_prices {
            if price < holding_price {
                holding_price = price;
                continue;
            }

            total_profit += price - holding_price;
            holding_price = price;
        }

        total_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);

        assert_eq!(Solution::max_profit(vec![2, 2, 4, 8]), 6);
        assert_eq!(Solution::max_profit(vec![2, 2, 4, 1]), 2);

        assert_eq!(Solution::max_profit(vec![1, 5]), 4);
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
