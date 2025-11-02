// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  LeetCodeではPremium問題なので、NeetCodeで代替
  https://neetcode.io/problems/count-connected-components
  問題の理解
  - 問題文と入出力を見てもさっぱりなので解説などを見る。
  https://www.youtube.com/watch?v=8f1XPm4WOUc

  - 解説を見てからの問題の理解
  ノードの数nと二次元配列edgesが与えられる。edgesは[[0,1],[0,2],[3,4]]のようになっておりエッジのリストである。
  エッジのリストとはどのノード同士がつながっているかという情報。
  連結成分(connected component)を戻り値として返す。
  例ではコンポーネント数が2になる。独立したグループが2つある。
    0   3
   / \  |
   1 2  4

  何がわからなかったか
  - まず問題文で何を聞かれているのかわからなかった。
  - 解説をみて問題文を理解したが、無向グラフの扱い方（データ構造）がよくわからなかった。
  - n,edgesが与えられるが、edgesに含まれるノードの種類よりも大きいnが入力されるのがよくわからない。単純に異常値として入力されている？

  何を考えて解いていたか
  - RustはNeetCodeの採点システムでジャッジできないので自分でテストコードを書いてジャッジする。ChatGPTにもテストコードを書いてもらう。
  - エッジの配列は昇順ソートされているように見えるが、問題の制約にその旨は無いのであてにしないほうが良さそう。
  - nでエッジの数を指定しているのはなぜだろう。
  edgesのnode番号からsetで一意なnode番号の数から求められると思ったが、必ずしもnで指定されるノードの数とedgesから求められるノードの数が同じではないということだろうか。

  想定ユースケース
  - SNSのユーザー同士のつながりをedges[i]で表現して、交友関係のグループ数を数える。（なんの役に立つかはよくわからないが）

  正解してから気づいたこと
  - edgesに含まれるノードの種類よりも大きいnが入力されるのは、エッジを持たないノードが存在しているという意味であることが分かった。
  n=3,edges=[[0,1]]の場合、0と1を繋ぐエッジがあり、ノードは[0,1,2]の三種類あることが考えられる。このとき2はedgesに出現せずエッジを持たないので孤立している。
  グラフにおいて2のようなエッジを持たない孤立したノードも自己のノードと連結しているコンポーネント(連結成分)の一つとして返す必要がある。
  https://ja.wikipedia.org/wiki/%E9%80%A3%E7%B5%90%E3%82%B0%E3%83%A9%E3%83%95#:~:text=%E6%A5%B5%E5%A4%A7%E3%81%A7%E9%80%A3%E7%B5%90%E3%81%AA%E9%83%A8%E5%88%86%E3%82%B0%E3%83%A9%E3%83%95%E3%81%AF%E3%80%81%E9%80%A3%E7%B5%90%E6%88%90%E5%88%86%EF%BC%88connected%20component%EF%BC%89%E3%81%A8%E3%81%84%E3%81%86%E3%80%82

  - 今回は採点システムが対応しておらず実装のテンプレートが無いので自分でメソッドのシグネチャを考える必要があり、以前型についてレビューを受けたことを思い出してかなり迷った。
  https://github.com/t9a-dev/LeetCode_arai60/pull/15#discussion_r2464752606
  具体的にはcount_components(n: <u32|usize>...)でu32にするかusizeにするかで迷った。
  公式ドキュメントの記述内容から配列のインデックスやポインタではない場面なのでu32を利用しようかと思った。
  しかし、改めて調べてみるとシグネチャを明示的に指定する場面では以下のような感じらしい。
  - その値がプロトコルの仕様などで厳密にデータサイズが決まっているときはu32などの固定長を使う。
  - クレートのAPIとしてはusizeとして公開するのが互換性の観点から自然。明らかにusizeも必要ない場合、パフォーマンスの観点からRustの整数値型デフォルトのi32又は無じサイズのu32を内部で利用するように実装している例があった。
    https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#23-29
    https://docs.rs/petgraph/latest/src/petgraph/graph_impl/mod.rs.html#105
  結論としてメソッドのシグネチャではusizeを利用する。
  問題の入力の制約から考えると2025年の一般的な実行環境(x86_64などの64bit環境)で実行した場合usizeは明らかに過剰だが内部でパフォーマンスのためにu32にキャストすることも過剰なので今回は行わない。
  外部に公開するAPIとして自分が設計するのであればusizeの方が互換性の観点から妥当であると考えた。
*/

use std::collections::{HashMap, HashSet};

pub struct Solution {}
impl Solution {
    pub fn count_components(n: usize, edges: Vec<Vec<usize>>) -> usize {
        let mut adjacency_nodes_by_node: HashMap<usize, Vec<usize>> = HashMap::new();

        for edge in edges {
            if let (Some(&a), Some(&b)) = (edge.get(0), edge.get(1)) {
                adjacency_nodes_by_node
                    .entry(a)
                    .and_modify(|nodes| nodes.push(b))
                    .or_insert(vec![b]);
                adjacency_nodes_by_node
                    .entry(b)
                    .and_modify(|nodes| nodes.push(a))
                    .or_insert(vec![a]);
            }
        }

        let mut components_count = 0;
        let mut visited_nodes: HashSet<_> = HashSet::new();

        for node in 0..n {
            if visited_nodes.contains(&node) {
                continue;
            }

            Self::explore_component(&mut adjacency_nodes_by_node, node, &mut visited_nodes);
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

        if let Some(adjacency_nodes) = adjacency_nodes_by_node.get(&node) {
            for node in adjacency_nodes {
                Self::explore_component(adjacency_nodes_by_node, *node, visited_nodes);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
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
    fn step1_empty_edges() {
        assert_eq!(Solution::count_components(4, vec![]), 4);
    }

    #[test]
    fn step1_loop_edges() {
        // 自己とつながる循環edgeが含まれる
        assert_eq!(
            Solution::count_components(3, vec![vec![0, 0], vec![1, 2], vec![1, 3]]),
            2
        );
    }

    /*
      以下、ChatGPT(GPT-5)で生成
    */
    #[test]
    fn step1_isolated_nodes() {
        // 孤立ノードが存在するケース
        // edges に現れないノード (3,4) が独立したコンポーネントになる
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step1_single_node() {
        // ノードが1つだけでedgeもない場合
        assert_eq!(Solution::count_components(1, vec![]), 1);
    }

    #[test]
    fn step1_no_nodes() {
        // ノード数0のとき（空グラフ）
        assert_eq!(Solution::count_components(0, vec![]), 0);
    }

    #[test]
    fn step1_disconnected_subgraphs() {
        // 離れたサブグラフが複数存在するケース
        assert_eq!(
            Solution::count_components(7, vec![vec![0, 1], vec![1, 2], vec![3, 4], vec![5, 6]]),
            3
        );
    }

    #[test]
    fn step1_self_loop_and_extra_nodes() {
        // 自己ループを含むが、それは単一ノード扱い。孤立ノードも存在。
        assert_eq!(
            Solution::count_components(4, vec![vec![0, 0], vec![1, 2]]),
            3
        );
    }

    #[test]
    fn step1_large_disconnected_with_gap() {
        // edgesの最大ノード番号がn-1未満で、間に孤立ノードがあるケース
        // ノード3が孤立している
        assert_eq!(
            Solution::count_components(5, vec![vec![0, 1], vec![2, 4]]),
            3
        );
    }
}
