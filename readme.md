# poc_slint

slint の練習用リポジトリ

## examples

### template.rs

[slint-ui/slint-rust-template: A template for a Rust Application using Slint. This is meant to be used with cargo-generate.](https://github.com/slint-ui/slint-rust-template)

- 複数の slint ファイルをインポート可能に
- anyhow への対応

### in_code.rs

template.rs を slint::slint マクロでコード内に実装

生成される構造体はエディタでのサポートは変わらない。生成されたコードを見るのが良さそう

### memory_game.rs

[はじめに - Slint メモリー ゲーム チュートリアル (Rust)](https://slint.dev/releases/1.3.2/docs/tutorial/rust/introduction)の実装する

## 課題

### 複数の Slint コンポーネントをビルドできない

[Create multiple components in generated Rust and C++ code · Issue #784 · slint-ui/slint](https://github.com/slint-ui/slint/issues/784)

コメントアウトで対応するかコードないマクロで作成するかしかなさそう

## 参考

- [slint-ui/slint-rust-template: A template for a Rust Application using Slint. This is meant to be used with cargo-generate.](https://github.com/slint-ui/slint-rust-template)
- [はじめに - Slint メモリー ゲーム チュートリアル (Rust)](https://slint.dev/releases/1.3.2/docs/tutorial/rust/introduction)
- [GUI フレームワーク Slint の紹介 #Rust - Qiita](https://qiita.com/task_jp/items/5e76f66366673d46afcd#rust-%E3%81%A8-slint-%E9%96%93%E3%81%A7%E5%85%B1%E6%9C%89%E3%81%99%E3%82%8B%E3%82%B0%E3%83%AD%E3%83%BC%E3%83%90%E3%83%AB%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%81%AE%E4%BD%9C%E6%88%90)
- [Frank1126lin/ImageShowWithSlint: use slint to show the image](https://github.com/Frank1126lin/ImageShowWithSlint/tree/master)
