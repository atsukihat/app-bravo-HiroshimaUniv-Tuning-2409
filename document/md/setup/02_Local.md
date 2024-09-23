# ローカル環境での開発

コンテストの評価対象となる点数は、こちらで用意した VM 環境上で行われたものに限られます。ただし、開発については各自 PC でのローカル環境での開発も可能です。

このドキュメントではローカル環境の開発環境構築について説明します。

## 注意事項

- 秘密鍵がない、あるいは権限が適切に設定されていない場合、スクリプトの実行に失敗することがあります。
- お使いのPC環境によってはローカルでの環境構築が困難な場合もあります。LinuxもしくはMacOSでの環境構築を前提としているため、特にWindowsユーザの場合はWSLを使用してUbuntuなどのLinux系のOSでの環境構築を勧めます。
- Dockerで使用しているイメージはAzureContainerRegistry(ACR)からプルしているため、Docker Hubの制限にかかることはありません。

## 事前準備

- 秘密鍵の用意や VM ドメインのフォーマットの確認は[最初にやること](./01_Start.md#手順ローカル)を参考にしてください。  
- ``benchmarker``と``webapp``はDockerで環境構築をするため、下記リンクからDocker Desktopのインストールをお願いします。
    - Mac：https://docs.docker.com/desktop/install/mac-install/
    - Windows：https://docs.docker.com/desktop/install/windows-install/

## 手順

1. ローカル環境に、 fork したリポジトリをクローンし、ルートディレクトリにある `init.sh` を実行してください。

    ```bash
    git clone https://github.com/atsukihat/app-bravo-HiroshimaUniv-Tuning-2409.git
    cd app-bravo-HiroshimaUniv-Tuning-2409
    bash init.sh 52.185.136.102 app-bravo.ftt2407.dabaas.net
    ```

    「初期化に成功しました。」という出力がされていることを確認し、http://localhost にアクセスして画面が表示されるか確認してみてください。

    **※初回起動時は docker build のキャッシュがないため、コンテナの起動に時間がかかる可能性があります。**

1. 評価スクリプトを動かしてみてください。

    ```bash
    $ bash run.sh
    ===================================================


    負荷試験が完了しました！！！
    あなたのスコア: 500

    より詳細な情報は下記ファイルをご覧ください
    ログファイル: ./benchmarker/logs/20240726_******.json
    負荷試験詳細ファイル: ./benchmarker/scores/raw-data-20240726_******.json
    スコアファイル: ./benchmarker/scores/score-20240726_******.json


    ===================================================
    ```

    実行が完了しスコアが出力されればローカルでの環境構築は完了です。

### ローカル環境と VM 環境の違い

- ローカル環境で、`entry.sh` は利用できません。VM 環境で最初に利用することを想定しています。
- ローカル環境では localhost を使って通信を行うため、負荷試験時に https 通信は行われません。
- ローカル環境と VM 環境で利用するアプリの compose ファイルは異なるので注意してください。ローカル環境で使用される compose ファイルは`webapp/docker-compose.local.yaml`です。
- ローカル環境はフロントエンド・バックエンドにホットリロードを有効化しており、それぞれ`webapp/frontend/src`と`webapp/backend/src`配下のファイルの変更についてはコンテナを再起動することなく変更が反映されます。ただし、ライブラリを新しくインストールしたり、NginxやMySQLの設定ファイルを反映させるためには再起動が必要です。
- ローカル環境では評価スクリプトを実施してもスコアを運営に送信しないため、コンテストの結果には影響しません。コンテストのスコアを更新したい場合は、VM 環境で評価スクリプトを実施してください。

---

[トップ](../../README.md)
