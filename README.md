# LeetCode_arai60
一般社団法人ソフトウェアエンジニアリング協会のコーディング練習会で利用するリポジトリ  
[問題集](https://1kohei1.com/leetcode/)  
番号順ではなくカテゴリ毎に解いていくのが良さそう

# 利用手順
- `0049_group_anagrams`みたいな感じでブランチを作成する
- Pull Requestのメッセージは以下のフォーマットで行う

```md
問題: [問題へのリンク](https://example.com)
次に解く問題: [次に解く問題へのリンク](https://example.com)
ファイルの構成: `./src/bin/<step1.rs|step2.rs|step3.rs>`
```

- Discordのレビュー依頼チャンネルに投稿する
``` md
問題: [問題へのリンク](https://example.com)
PR: [GitHub Pull Requestへのリンク](https://example.com)
言語: Rust
直近で同じ問題を解いている5人をメンションする(Discordの検索で問題へのリンクから検索するのが良さそう)
```

# 練習方法
    答えを見ずに考えて、5分考えて分からなかったら答えを見る
    答えを見て理解したと思ったら、答えを隠して書く
    筆が進まず5分迷ったら答えを見る
    見ちゃったら一回全部消してやり直し答えを送信して、正解になったら、まずは一段階目
    次にコードを読みやすくするようにできるだけ整える これで動くコードになったら二段階目です。
    そしたらまた全部消す
    今度は、時間を測りながら、もう一回、書く
    書いてアクセプトされたら文字消してもう一回書く
    これを10分以内に一回もエラーを出さずに書ける状態になるまで続ける3回続けてそれができたらその問題はひとまず丸
    3ステップ目まで終わったタイミングで講師陣にレビューを依頼し、レビューを元にコードを書き直す
    LetCodeの問題とは直接関係ないが、関連するライブラリの再実装などするのもよい。たとえば、@lru_cacheやheapqライブラリなどを使用した解法があったのでLRU Cacheの実装やBinary Heapの実装なども行う。
https://github.com/irohafternoon/LeetCode/blob/49.-Group-Anagrams/README.md#%E7%B7%B4%E7%BF%92%E6%96%B9%E6%B3%95

# 実行方法
### コードの実行
`cargo run --bin step1`

### テストの実行
`cargo test --bin step1`

# 参考リンク
## 既にRustで実践しているリポジトリ
- https://github.com/Yoshiki-Iwasa/Arai60
