// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  問題の理解
  - 昇順にソートされた一意の整数を含む配列を回転させたものが入力として与えられる。配列中に含まれる最小の値を答えとして返す。
  - 時間計算量O(log n)のアルゴリズムで実装することが制約。
    - 時間計算量O(log n)を満たすことが必須なので、入力の長さなどから事前に時間計算量を見積もる必要はない。
  - 回転させた配列について
    - 4回転のとき [0,1,2,4,5,6,7] -> [4,5,6,7,0,1,2]
    - 7回転のとき [0,1,2,4,5,6,7] -> [0,1,2,4,5,6,7]

  何を考えて解いていたか
  - 時間計算量O(log n)を満たすために二分探索アルゴリズムで実装する必要がある。
    - しかし、入力の配列はn回回転したものが渡され、昇順ソートされている状態とは限らない。
  - 最小値を保持する変数を作っておき、探索を行いながら最小値を見つけるたびに更新する。探索終了後に最小値を返す。
  - ソート処理を実行するとO(n log n)となり、問題の制約を満たせない。
  時間切れなので答えを見る

  何がわからなかったか
  - 何回回転したかが示されない状態で、入力をソートされた状態に戻す方法が分からなかった。

  解答の理解
  https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/solutions/7178461/153-find-minimum-in-rotated-sorted-array-6m7g/?envType=problem-list-v2&envId=vnnqktms
  - ソートされている配列を回転しても、ある区間(まとまり)はソートされている状態が保持されている。この点に注目している。
  - ある時点で見ている区間の左端の値　< 右端の値　が成立していれば、この区間の最小値が区間の左端であることが分かる。
    - 最初に配列全体がこの条件を満たしていれば、配列がソートされていることが分かるので、早期リターンnums[0]できる。
  - 右端の方が小さければ、最小値と比較して更新できるか試してから右端を飛ばして次の未探索領域を見る。

  正解してから気づいたこと
  - 解き方を知っているかどうかという感じがした。
    - 回転という操作に惑わされず、区間（まとまり）に注目してソートされている状態が維持されているという点に気付くことができるかどうか。
*/

pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("nums must not be empty");
        }

        if nums.first().unwrap() <= nums.last().unwrap() {
            return *nums.first().unwrap();
        }

        // [start,end)
        // start <= i < end
        let mut start = 0;
        let mut end = nums.len();
        let mut min_value = i32::MAX;

        while start < end {
            let middle = start + (end - start) / 2;
            let middle_value = nums[middle];
            let start_value = nums[start];

            if start_value <= middle_value {
                min_value = min_value.min(start_value);
                start = middle + 1;
            } else {
                min_value = min_value.min(middle_value);
                end = middle;
            }
        }

        min_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1_test() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);

        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    #[should_panic]
    fn step1_empty_nums_test() {
        let empty_nums = Vec::new();
        Solution::find_min(empty_nums);
    }
}
