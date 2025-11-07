// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時に考えたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - split_atメソッドの方が分かりやすい(又は使うべき)と言われるかも

  他の人のコードを読んで考えたこと
  - 閉区間・開区間・半開区間について。
  閉区間(closed interval): [a,b] a <= x <= b
  開区間(open interval): (a,b) a < x < b
  半開区間(half-open interval): 半開区間はどちらでもある。
    左開右閉区間(left-open,right-closed interval) (a,b] a < x <= b
    左閉右開区間(left-closed,right-open interval) [a,b) a <= x < b
  自分の言葉で説明すると
    - 開区間(open interval)はa,bを内に閉じ込めない（開いている）ので区間に含まない
    - 閉区間(closed interval)はa,bを内に閉じ込める（閉じている）ので区間に含める
  https://github.com/ryosuketc/leetcode_arai60/pull/24/files#r2122118285
  https://www.optics-words.com/english_for_science/interval.html

  - 半開区間は使うときに注意が必要。上記のように左、右どちらが開閉しているのかが曖昧な用語であるため。
  https://discord.com/channels/1084280443945353267/1301587996298182696/1361727443781550322

  - 配列の真ん中を表す変数名について。
  自分はcenterとしていたがmiddleという命名をよく見かけるなと思った。
  https://github.com/ryosuketc/leetcode_arai60/pull/24/files#diff-3de77ea0c72d053fe618183020f52337a0f4dbeb9bdc641cb2100f5b8a25c28bR12
  https://github.com/Satorien/LeetCode/pull/24/files#diff-163e663c7aabe5df9067742ff97da70680736e53199df9d9c07577d25e0ca3e2R42

  - centerとしている人も見かけたがstep2からmidにしているようだった。
  https://github.com/h1rosaka/arai60/pull/27/files#diff-7e1a92af8dc65ffab400b9bf2416693d15f370a5cfb6ac37e009f85a67c03a32R14

  - 問題には直接関係ないがsetattrに関連して脆弱性の話題が出てきており面白いなと思った。
  https://github.com/docto-rin/leetcode/pull/23/files#r2437715955

  レビューコメントにもあったが自由度が高くて強力な道具を見かけたら一度立ち止まるのが良さそう。
  setattrはオブジェクトに文字列で指定された属性と値を割り当てられる組み込み関数。
  https://docs.python.org/ja/3.13/library/functions.html#setattr

  JavaScriptでは文字列をそのままスクリプトとして解釈するeval()があり、MDNでは明示的に使うなと書かれていた。
  https://developer.mozilla.org/ja/docs/Web/JavaScript/Reference/Global_Objects/eval#%E7%9B%B4%E6%8E%A5_eval_%E3%82%92%E4%BD%BF%E3%82%8F%E3%81%AA%E3%81%84%E3%81%A7%E3%81%8F%E3%81%A0%E3%81%95%E3%81%84!

  改善する時に考えたこと
  - nums_center -> nums_middle_index
    nums_centerだけだと真ん中から値を取り出したのか配列の添字なのかが明確でない。
    centerは幾何学的・座標における中心に使うのが適切で配列における中央を指し示すならmiddleの方が自然らしい。(by GPT-5)
  - .to_vec()で配列のcloneが発生するので、ヘルパーメソッドに切り出してシグネチャで参照を受け取るようにする。
  - node.borrow_mut()は明示的にスコープを切って存続期間を短くする。(by GPT-5)
  - node.borro_mut().left = で再帰処理を実行しているが、borrwo_mut()の存続期間が長いので、Rust的には忌避される書き方らしい(by GPT-5)
  - メソッドの返り値がOption<T>のときに、let Some(nums_middle_vlaue)　= nums.get()のelseは?で置き換えられる。(by GPT-5)
  これに関してはResult型でしか使えないと思っていたのでかなり勉強になった。
  - これは不採用　-> [..nums_center],[nums_middle+1..]の部分はsplit_at(num_center)で置き換えられる。(by GPT-5)
  let (left, right) = nums.split_at(nums_middle);とするとnums_middleがrightの左閉区間になるので、leftに対してright[1..]とする必要があり分かりづらいと思った。
  let (left, right) = (nums[..nums_middle], nums[nums_middle+1..]);の方が宣言的で分かりやすいと思った。
*/

use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_binary_search_tree(&nums)
    }

    fn build_binary_search_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let nums_middle_index = nums.len() / 2;
        let node_val = nums.get(nums_middle_index)?;
        let (left_nums, right_nums) = (&nums[..nums_middle_index], &nums[nums_middle_index + 1..]);
        let left_node = Self::build_binary_search_tree(left_nums);
        let right_node = Self::build_binary_search_tree(right_nums);

        let node = Rc::new(RefCell::new(TreeNode::new(*node_val)));
        {
            let mut node = node.borrow_mut();
            node.left = left_node;
            node.right = right_node;
        } // ここでスコープを抜けるとlet mut node = node.borrow_mut()が破棄されるので、borrow_mut()の存続期間が最小になる。

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn step2_test() {
        let node_values = vec![-10, -3, 0, 5, 9];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)];
        assert_eq!(actual, expect);

        let node_values = vec![3, 1];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(1), Some(3)];
        assert_eq!(actual, expect);

        let node_values = vec![1];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![Some(1)];
        assert_eq!(actual, expect);

        let node_values = vec![];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![];
        assert_eq!(actual, expect);

        let node_values = vec![-15, -10, -3, 0, 5, 9, 6];
        let actual = binary_tree_to_vec(&Solution::sorted_array_to_bst(node_values));
        let expect = vec![
            Some(0),
            Some(-10),
            Some(9),
            Some(-15),
            Some(-3),
            Some(5),
            Some(6),
        ];
        assert_eq!(actual, expect);
    }

    fn vec_to_binary_tree(node_values: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(root_value) = node_values.get(0).and_then(|v| *v) else {
            return None;
        };

        let root = Rc::new(RefCell::new(TreeNode::new(root_value)));
        let mut nodes = VecDeque::new();
        nodes.push_back(Rc::clone(&root));

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            let mut node = node.borrow_mut();
            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let left_node = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.left = Some(Rc::clone(&left_node));
                nodes.push_back(left_node);
            }
            i += 1;

            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                let right_node = Rc::new(RefCell::new(TreeNode::new(node_val)));
                node.right = Some(Rc::clone(&right_node));
                nodes.push_back(right_node);
            }
            i += 1;
        }

        Some(root)
    }

    fn binary_tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut node_values = vec![];
        let mut nodes = VecDeque::new();
        nodes.push_back(root.as_ref().map(Rc::clone));

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            node_values.push(Some(node.borrow().val));
            nodes.push_back(node.borrow().left.as_ref().map(Rc::clone));
            nodes.push_back(node.borrow().right.as_ref().map(Rc::clone));
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn step2_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(
            binary_tree_to_vec(&vec_to_binary_tree(&node_values)),
            node_values
        );
    }
}
