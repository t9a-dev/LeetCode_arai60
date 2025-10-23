// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 空でない一桁の整数を値として持つ2つのリンクリストが与えられるので、それぞれの値を加算してリンクリストとして返す。
  l1    = [2,4,3]
  l2    = [5,6,4]
  output= [7,0,8]

  何がわからなかったか
  - 条件を書き下す以外の解法

  何を考えて解いていたか
  - ナイーブな実装しか思い浮かばないのでこれでやる。
    - l1,l2から値を取り出して配列にする。
    - 配列[i]に対応する数値を足していって結果の配列を求める。
      - 加算無限ループの終了条件
        - リンクリストから取り出した値が両方とも空で、繰り上げもない場合。
        - 繰り上げがなく、合計値も0の場合
      - 10以上になったときに次の桁に繰り上げする必要がある。繰り上げがあるとき次で+1する
    - 結果の配列をリンクリストにして返す。

  正解してから気づいたこと
  - 実装がナイーブすぎる気がするが、よりすっきりと書く方法が思い浮かばないので、step2で他の人の実装を確認する。
*/

pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1_values = Self::list_node_to_vec(&l1);
        let l2_values = Self::list_node_to_vec(&l2);
        let mut result_values: Vec<_> = Vec::new();
        let mut i = 0;
        let mut carry = 0;

        loop {
            let (l1_val, l2_val) = (l1_values.get(i), l2_values.get(i));

            if carry == 0 && l1_val.is_none() && l2_val.is_none() {
                break;
            }

            let (l1_val, l2_val) = (l1_val.unwrap_or(&0), l2_val.unwrap_or(&0));
            let mut sum = l1_val + l2_val + carry;
            carry = 0;

            if sum == 0 {
                result_values.push(sum);
                i += 1;
                continue;
            }

            if 10 <= sum {
                sum = sum - 10;
                carry = 1;
                i += 1;
                result_values.push(sum);
                continue;
            }

            if sum == 0 && carry == 0 {
                break;
            }

            result_values.push(sum);
            i += 1;
        }

        Self::vec_to_list_node(&result_values)
    }

    pub fn vec_to_list_node(values: &Vec<i32>) -> Option<Box<ListNode>> {
        values.into_iter().rev().fold(None, |child, v| {
            let mut parent = Box::new(ListNode::new(*v));
            parent.next = child;
            Some(parent)
        })
    }

    pub fn list_node_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut current_node = head;

        while let Some(node) = current_node {
            out.push(node.val);
            current_node = &node.next;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        let (l1_values, l2_values) = (vec![2, 4, 3], vec![5, 6, 4]);
        let (l1, l2) = (
            Solution::vec_to_list_node(&l1_values),
            Solution::vec_to_list_node(&l2_values),
        );
        assert_eq!(Solution::list_node_to_vec(&l1), l1_values);
        assert_eq!(Solution::list_node_to_vec(&l2), l2_values);
        assert_eq!(
            Solution::list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![7, 0, 8]
        );

        let (l1_values, l2_values) = (vec![0], vec![0]);
        let (l1, l2) = (
            Solution::vec_to_list_node(&l1_values),
            Solution::vec_to_list_node(&l2_values),
        );
        assert_eq!(Solution::list_node_to_vec(&l1), l1_values);
        assert_eq!(Solution::list_node_to_vec(&l2), l2_values);
        assert_eq!(
            Solution::list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![0]
        );

        let (l1_values, l2_values) = (vec![9, 9, 9, 9, 9, 9, 9], vec![9, 9, 9, 9]);
        let (l1, l2) = (
            Solution::vec_to_list_node(&l1_values),
            Solution::vec_to_list_node(&l2_values),
        );
        assert_eq!(Solution::list_node_to_vec(&l1), l1_values);
        assert_eq!(Solution::list_node_to_vec(&l2), l2_values);
        assert_eq!(
            Solution::list_node_to_vec(&Solution::add_two_numbers(l1, l2)),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
