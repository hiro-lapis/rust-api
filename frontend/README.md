# frontend

蔵書管理アプリケーションの画面側を立ち上げることができます。画面を使いながら実装を進めたい、あるいは、実装が終わったのでどういった結果になったのかを見てみたいという方は、下記の手順に従ってこのアプリケーションを立ち上げてください。

## Node.js のインストール

本書の 2 章でも説明しましたが、前提として Node.js がご自身の環境にインストールされている必要があります。2 章の案内あるいは下記サイトより、Node.js をインストールしてください。

- https://nodejs.org/en/download/package-manager

## 環境変数ファイルの設定

APIとの疎通に環境変数が必要なので下記コマンドを実行  

```
cp sample.env.local .env
```

### 

ログイン画面にて [net::ERR_EMPTY_RESPONSE](img/front-end-request-error.png) というエラーが出た場合は環境変数を下記のように書き換えましょう。  

```
- API_ROOT_URL=localhost
+ API_ROOT_URL=0.0.0.0
```

APIコンテナでLISTENしているlocalhostが端末それ自体のlocalhostと繋がってないため名前解決できず発生しています。  
フロントエンドアプリケーションは 0.0.0.0 でLISTENすると解決できます。  
[参考記事](https://qiita.com/amuyikam/items/01a8c16e3ddbcc734a46)  

＊あくまで開発環境での設定です。リモート環境においてはインフラの設定に合わせてドメインを指定するようにしましょう。  


## 画面の立ち上げ

ローカルで立ち上げる場合、次の手順で立ち上がります。

```shell
$ npm install
$ npm run dev
```

最後にブラウザで `localhost:3000` と入力すると、画面が表示されます。
