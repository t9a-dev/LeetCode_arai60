// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  V = n
  E = edges.len()
  時間計算量: O(V + E) n回ループを回している。edgesのサイズ分ループを回している。
  空間計算量: O(V + E) 隣接リスト(adjacency_nodes_by_node)のメモリ利用量。n回再帰処理を呼び出してスタックフレームをスタックに積んでいる。
*/

/*
  1回目: 9分6秒
  2回目: 5分6秒
  3回目: 5分29秒
*/

/*
  所感
  - 1回目で10分ギリギリになったのが不思議だったので少し立ち止まって考えたことのメモ。
    - 1回目は時間ばかり気にしてなるべく早く書こうとして9分程度かかった。2回目は逆にゆっくり段階を踏んで書くようにしてみたところ5分程度で書けた。
      - 急いで書くと思考がまとまり切る前にコードを書くようなことが起こり、書くだけ書いたコードの修正やタイプミスに時間を取られている。
      - 2回目はゆっくりと段階を踏みながら考えて書く方法を取ったので、主観的な時間ではだいぶ時間を使っているように感じたが結果はそうでなかった。
*/

use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    pub fn count_components(n: usize, edges: Vec<Vec<usize>>) -> usize {
        let mut adjacency_nodes_by_node: HashMap<usize, Vec<usize>> = HashMap::new();
        for edge in edges {
            if let (Some(&node1), Some(&node2)) = (edge.get(0), edge.get(1)) {
                adjacency_nodes_by_node
                    .entry(node1)
                    .and_modify(|nodes| nodes.push(node2))
                    .or_insert(vec![node2]);
                adjacency_nodes_by_node
                    .entry(node2)
                    .and_modify(|nodes| nodes.push(node1))
                    .or_insert(vec![node1]);
            }
        }

        let mut components_count = 0;
        let mut visited_nodes: HashSet<usize> = HashSet::new();
        for node in 0..n {
            if visited_nodes.contains(&node) {
                continue;
            }

            Self::explore_component(&adjacency_nodes_by_node, node, &mut visited_nodes);
            components_count += 1;
        }

        components_count
    }

    fn explore_component(
        adjacency_nodes_by_node: &HashMap<usize, Vec<usize>>,
        node: usize,
        visited_nodes: &mut HashSet<usize>,
    ) {
        if !visited_nodes.insert(node) {
            return;
        }

        if let Some(nodes) = adjacency_nodes_by_node.get(&node) {
            for node in nodes {
                Self::explore_component(adjacency_nodes_by_node, *node, visited_nodes);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step3_test() {
        // nで指定されるnodeが全てedgeに現れる
        assert_eq!(
            Solution::count_components(3, vec![vec![0, 1], vec![0, 2]]),
            1
        );
        // nで指定されるnodeが全てedgeに現れる
        assert_eq!(
            Solution::count_components(6, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![4, 5]]),
            2
        );
        // nで指定されるnodeの一部がedgesに現れる
        assert_eq!(
            Solution::count_components(6, vec![vec![0, 1], vec![0, 2], vec![0, 3]]),
            3
        );
    }

    #[test]
    fn step3_empty_edges() {
        // nで指定される全てのノードが孤立している
        assert_eq!(Solution::count_components(4, vec![]), 4);
    }

    #[test]
    fn step3_loop_edges() {
        // 自己とつながる循環edgeを持つnodeが含まれる
        assert_eq!(
            Solution::count_components(3, vec![vec![0, 0], vec![1, 2], vec![1, 3]]),
            2
        );
    }

    /*
      以下、ChatGPT(GPT-5)で生成
    */
    #[test]
    fn step3_isolated_nodes() {
        // 孤立ノードが存在するケース
        // edges に現れないノード (3,4) が独立したコンポーネントになる
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step3_single_node() {
        // ノードが1つだけでedgeもない場合
        assert_eq!(Solution::count_components(1, vec![]), 1);
    }

    #[test]
    fn step3_no_nodes() {
        // ノード数0のとき（空グラフ）
        assert_eq!(Solution::count_components(0, vec![]), 0);
    }

    #[test]
    fn step3_disconnected_subgraphs() {
        // 離れたサブグラフが複数存在するケース
        assert_eq!(
            Solution::count_components(7, vec![vec![0, 1], vec![1, 2], vec![3, 4], vec![5, 6]]),
            3
        );
    }

    #[test]
    fn step3_self_loop_and_extra_nodes() {
        // 自己ループを含むが、それは単一ノード扱い。孤立ノードも存在。
        assert_eq!(
            Solution::count_components(4, vec![vec![0, 0], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step3_large_disconnected_with_gap() {
        // edgesの最大ノード番号がn-1未満で、間に孤立ノードがあるケース
        // ノード3が孤立している
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![2, 4]]),
            3
        );
    }
}
