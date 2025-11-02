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
  他の人のコードを読んで考えたこと
  - UnionFindを利用した解法。
  https://github.com/plushn/SWE-Arai60/pull/19/files#diff-e0cc2f02675fcae93349b69154357d2982f7ae61a03d96fb7be8b79be493eaffR116
  https://github.com/docto-rin/leetcode/pull/28/files#diff-4f6b01b75cf61fa706e6463e0a6840a6a0685f9f0cfcc46cc7dfb3530e908b18R64

  - UnionFindは微妙に常識から外れるそうなので今はArai60を完走することを優先してスキップ。
  https://github.com/docto-rin/leetcode/pull/28/files#diff-4f6b01b75cf61fa706e6463e0a6840a6a0685f9f0cfcc46cc7dfb3530e908b18R66-R71

  - 反復深化法という知らない単語がでてきた。深さ優先探索と幅優先探索のハイブリッドだと理解した。
  パスの長さが深い場合、深さ優先探索を行うとメモリに乗り切らないという問題がある。深さを制限しつつ幅優先探索を行うことで、限られた時間内で大まかな結果を返すといった使い方があると理解した。
  こういった手法も存在するといった程度にとどめておく。
  https://github.com/yas-2023/leetcode_arai60/pull/19#discussion_r2443191681
  https://ja.wikipedia.org/wiki/%E5%8F%8D%E5%BE%A9%E6%B7%B1%E5%8C%96%E6%B7%B1%E3%81%95%E5%84%AA%E5%85%88%E6%8E%A2%E7%B4%A2

  - 計算量の見積もりで使われる記号でV,Eが使われている。グラフを扱う問題なので頂点の数(Vertex),エッジの数(Edge)から来ていると理解した。
  https://github.com/docto-rin/leetcode/pull/28/files#diff-4f6b01b75cf61fa706e6463e0a6840a6a0685f9f0cfcc46cc7dfb3530e908b18R78-R82

  改善する時に考えたこと
  - 隣接ノードのマップを作成するときにedge[0],edge[1]をa,bとしていたが、雑すぎるのでnode1,node2くらいのほうが良さそう
  - 変数宣言と、この変数を扱うforの間に改行は無い方が処理単位のまとまりが分かりやすいと思った。
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
    fn step2_test() {
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
    fn step2_empty_edges() {
        assert_eq!(Solution::count_components(4, vec![]), 4);
    }

    #[test]
    fn step2_loop_edges() {
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
    fn step2_isolated_nodes() {
        // 孤立ノードが存在するケース
        // edges に現れないノード (3,4) が独立したコンポーネントになる
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step2_single_node() {
        // ノードが1つだけでedgeもない場合
        assert_eq!(Solution::count_components(1, vec![]), 1);
    }

    #[test]
    fn step2_no_nodes() {
        // ノード数0のとき（空グラフ）
        assert_eq!(Solution::count_components(0, vec![]), 0);
    }

    #[test]
    fn step2_disconnected_subgraphs() {
        // 離れたサブグラフが複数存在するケース
        assert_eq!(
            Solution::count_components(7, vec![vec![0, 1], vec![1, 2], vec![3, 4], vec![5, 6]]),
            3
        );
    }

    #[test]
    fn step2_self_loop_and_extra_nodes() {
        // 自己ループを含むが、それは単一ノード扱い。孤立ノードも存在。
        assert_eq!(
            Solution::count_components(4, vec![vec![0, 0], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step2_large_disconnected_with_gap() {
        // edgesの最大ノード番号がn-1未満で、間に孤立ノードがあるケース
        // ノード3が孤立している
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![2, 4]]),
            3
        );
    }
}
