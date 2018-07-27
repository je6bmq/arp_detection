# ARPパケットを検出する
とよぎぃ通信 vol.8 の「RustでAmazon Dash ButtonのARPパケットを検出する」に掲載されているコードです．

Windowsで動作させる場合はpnetクレートを使うために設定が必要です．詳細は[pnetクレート]https://github.com/libpnet/libpnet#windows)を参照ください．

Rustコンパイラ群を以下のコマンドで導入して，

```
$ curl https://sh.rustup.sh.rs  -sSf | sh
```

main.rs内の９行目から１４行目における"XX"をMACアドレスの適切な値に設定することで，

以下のコマンドで実行可能です．

```
$ cargo run 
```