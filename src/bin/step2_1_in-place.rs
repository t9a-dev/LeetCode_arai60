// Step2_1_in-place
// 目的: インプレイスによる解法を実装しておく

/*
  - インプレイスでの実装を理解するのに以下の実装を参考にすることにして少し手を加えた。
  https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/6940535/somehow-elegant-iterative-solution-without-unwrap-and-clone-100/

  - 先頭にダミーのノードを入れることで先頭から重複しているときに対応している。
  このようにダミーの要素を計算のために追加することを「番兵」と呼ぶことが分かった。

  - 問題の理解
    cursor = sentinel_head(先頭番兵ノード)
    cursor.next = node[i](初回はhead)
    node[i].next = node[i+1](初回はheadの次ノード)
    -カーソルが指しているnode[i].valとnode[i+1].valが重複している間ループに入る。
      - 重複検知した時点でフラグを立てておく。
      - 次ノードの次ノードをカーソルノードの次ノードの参照先に更新する。
        - node[i].next = node[i+1].next
      - ループの先頭に戻る
    - 重複が発生していたかをフラグから判断
      - 重複が発生していたら、カーソルの指し先の次ノードをカーソルの指し先にする
        - cursor.next = node[i].next
      - 重複が発生していなければ、カーソルの指し先を一つ次に進める。
        - cursor = node[i]

  - 感想
  - 他の人の実装を見ているときにtailという変数名に違和感を覚えてしまい、解法のロジックが理解できない現象に遭遇した。
    - 自分の中でtailは末尾を想起するものである。
    - 入力として与えられたリンクリストのあるノードを指してtailといっているのは、末尾ではないだろうと考えてしまう。
    - どうしても納得できなかったので、ChatGPTに相談して変数名をwrite_cursorにすることにした。
  - 学習目的でなるべく理解できるまで粘って実装してみたが、基本的にはHashMapを利用した実装をすると思った。
    - アルゴリズムへの対応力が不足しているだけかも知れないが、読み書きにだいぶ時間と体力を消費している。
      - HashMapを利用した実装であれば思いついた解法をそのまま書いて空間計算量O(N)
      - インプレイスでの実装では時間と考える体力をそれなりに消費して空間計算量O(1)
      - 時間計算量はどちらもO(N)
        - 今の自分にとっては空間計算量O(1)に改善するために時間と体力を使いすぎるので、HashMapを利用した実装を採用すると思った。
          - 選択肢としてより空間計算量が優れる実装方法を調べながら実装できること自体には大きな意義があると思った。
        - インプレイスでの実装を時間と体力をあまり使わずに実装できることがソフトウェアエンジニアとしての常識なのかは気になるところ。
*/

/*
  Nは入力のサイズとする
  時間計算量: O(N)
  空間計算量: O(1)
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel_head = Box::new(ListNode { val: 0, next: head });
        let mut write_cursor = sentinel_head.as_mut();

        while let Some(mut node) = write_cursor.next.take() {
            let mut has_duplicate = false;

            while let Some(next_node) = node.next.as_mut() {
                // 重複していないのでループを抜ける
                if node.val != next_node.val {
                    break;
                }

                has_duplicate = true;
                // 次のノードと重複しているので飛ばして、次の次のノードを次のノードにする。
                node.next = next_node.next.take();
                continue;
            }

            if has_duplicate {
                // 現在のノードは重複している値を持つので飛ばして次のノードと入れ替える。
                write_cursor.next = node.next;
                continue;
            }

            // 重複していないのでwrite_cursorを次のノードに移動する。
            write_cursor = write_cursor.next.insert(node);
        }

        sentinel_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_to_list_node(values: &Vec<i32>) -> Option<Box<ListNode>> {
        values.into_iter().rev().fold(None, |child, v| {
            let mut parent = Box::new(ListNode::new(*v));
            parent.next = child;
            Some(parent)
        })
    }

    fn list_node_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut next_node = head;

        while let Some(node) = next_node {
            out.push(node.val);

            next_node = &node.next;
        }

        out
    }

    #[test]
    fn step2_1_test() {
        let source_vec = vec![1, 1, 2];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![2]
        );

        let source_vec = vec![1, 1, 2, 3, 3];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(
            list_node_to_vec(&Solution::delete_duplicates(head)),
            vec![2]
        );

        let source_vec = vec![1, 1, 1];
        let head = vec_to_list_node(&source_vec);
        assert_eq!(list_node_to_vec(&head), source_vec);
        assert_eq!(list_node_to_vec(&Solution::delete_duplicates(head)), vec![]);
    }
}
