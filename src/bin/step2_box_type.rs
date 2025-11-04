// Step2_box_type
// 目的: Rc<RefCell<TreeNode>> から Box<TreeNode>へ型を変更して実装できるか試してみる

/*
  本ファイルのコードはRust言語自体の学習を目的としています。LeetCodeテンプレートと型が一致しないため、採点システムを通りません。

  step2.rsからの変更点
  - LeetCodeのテンプレート上はTreeNodeの型がOption<Rc<RefCell<TreeNode>>>となっているが、この問題では内部可変性(RefCell)が必要ないのでOption<Box<TreeNode>>にした。
  ノード自体に変更を加えないので内部可変性(RefCell)が必要ないのではと思い実験のために実装した。

  - to_tree_nodes内でBox<T>特有の取り回しで躓いたので、ChatGPT(GPT-5)に聞いた。インラインコメント参照。

  所感
  - max_depthメソッドではTreeNodeに変更を加えていないので、内部可変性が必要ないことに気づけたのが良かった。
  しかし、Box<T>に変更した後に、参照の取り回し部分を自分で解決できない箇所が発生してChatGPT(GPT-5)に聞いた。（to_tree_nodesメソッド内のインラインコメント参照）
*/

use std::collections::VecDeque;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        /*
          本ファイルのコードはRust言語自体の学習を目的としています。LeetCodeテンプレートと型が一致しないため、採点システムを通りません。
        */
        let mut max_depth = 0;
        let mut frontiers = VecDeque::new();

        frontiers.push_back((root.as_ref(), 1));
        while let Some((node, depth)) = frontiers.pop_front() {
            let Some(node) = node else {
                continue;
            };

            max_depth = depth;

            frontiers.push_back((node.left.as_ref(), depth + 1));
            frontiers.push_back((node.right.as_ref(), depth + 1));
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    fn to_tree_nodes(node_values: &Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if node_values.is_empty() || node_values[0].is_none() {
            return None;
        };

        let mut root = Box::new(TreeNode::new(node_values[0].unwrap()));
        let mut nodes = VecDeque::new();
        nodes.push_back(root.as_mut());

        let mut i = 1;
        while let Some(node) = nodes.pop_front() {
            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                // ここで躓いてChatGPT(GPT-5)に聞いた。
                // 変数に束縛せず、直接node.leftに入れたあとに、node.leftから可変参照を取り出している。
                node.left = Some(Box::new(TreeNode::new(node_val)));
                nodes.push_back(node.left.as_deref_mut().unwrap());
            }
            i += 1;

            if let Some(node_val) = node_values.get(i).and_then(|v| *v) {
                // ここで躓いてChatGPT(GPT-5)に聞いた。
                // 変数に束縛せず、直接node.rightに入れたあとに、node.rightから可変参照を取り出している。
                node.right = Some(Box::new(TreeNode::new(node_val)));
                nodes.push_back(node.right.as_deref_mut().unwrap());
            }
            i += 1;
        }

        Some(root)
    }

    fn to_node_values(root: &Option<Box<TreeNode>>) -> Vec<Option<i32>> {
        let mut node_values = Vec::new();
        let mut nodes = VecDeque::new();
        nodes.push_back(root);

        while let Some(node) = nodes.pop_front() {
            let Some(node) = node else {
                node_values.push(None);
                continue;
            };

            node_values.push(Some(node.val));
            nodes.push_back(&node.left);
            nodes.push_back(&node.right);
        }

        while node_values.last().is_some_and(|v| v.is_none()) {
            node_values.pop();
        }

        node_values
    }

    #[test]
    fn step2_box_type_helper_method_test() {
        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(to_node_values(&to_tree_nodes(&node_values)), node_values);
    }

    #[test]
    fn step2_box_type_test() {
        let node_values = vec![Some(3)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 1);

        let node_values = vec![Some(3), None, Some(5)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 2);

        let node_values = vec![];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 0);

        let node_values = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        assert_eq!(Solution::max_depth(to_tree_nodes(&node_values)), 3);
    }
}
