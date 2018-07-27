# ARPパケットを検出する
とよぎぃ通信 vol.8 の「RustでAmazon Dash ButtonのARPパケットを検出する」に掲載されているコードです．

Windowsではpnetクレートを使うために設定が必要なため，[pnetクレート](https://github.com/libpnet/libpnet)を参照ください．

rustupを以下のコマンドで導入して，

```
$ curl -sSf https://rustup.sh.rs | sh
```

main.rs内の９行目から１４行目における"XX"をMACアドレスの適切な値に設定することで，

以下のコマンドで実行可能です．

```
$ cargo run 
```