// Actix-webの必要なコンポーネントをインポート
use actix_web::{error::ResponseError, middleware::Logger, web, App, HttpResponse, HttpServer};

// once_cellを使用してグローバルな正規表現オブジェクトを一度だけ初期化するためのLazy型をインポート
use once_cell::sync::Lazy;

// 乱数生成と関連のトレイトをインポート。ここでは、トップレベルの rng() と Rng トレイトを利用する
use rand::{rng, Rng};

// 正規表現を扱うためのRegexをインポート
use regex::Regex;

// エラーメッセージの表示のためにfmtモジュールをインポート
use std::fmt;

// 正規表現オブジェクトをLazyで初期化する。
// この正規表現は、"3d6" のような形式にマッチし、最初のキャプチャグループがロール数、
// 2番目のグループがダイスの面数を表す
static DICE_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\d+)d(\d+)$").expect("Invalid regex"));

// カスタムエラー型 MyError を定義。APIのエラー時に利用する
#[derive(Debug)]
struct MyError {
    msg: String,
}

// MyError に Display トレイトを実装して、人間にわかりやすいエラーメッセージを表示できるようにする
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // エラーメッセージを出力
        write!(f, "{}", self.msg)
    }
}

// MyError に標準の Error トレイトを実装することで、他のエラー処理と統合しやすくする
impl std::error::Error for MyError {}

// Actix-web の ResponseError トレイトを実装することで、MyError を HTTP レスポンスに変換可能にする
impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        // エラー発生時は HTTP 400 (Bad Request) を返し、エラーメッセージをレスポンスボディに含める
        HttpResponse::BadRequest().body(self.msg.clone())
    }
}

// ダイスロール処理を行う非同期関数
// URLパスから取得した文字列を解析して、ダイスの数値を生成する
async fn roll_dice(info: web::Path<String>) -> Result<HttpResponse, MyError> {
    // リクエストパスの値が正規表現にマッチするか確認
    if let Some(caps) = DICE_PATTERN.captures(&info) {
        // キャプチャグループからロール数（例："3"）を usize にパースする
        let rolls = caps[1].parse::<usize>().map_err(|_| MyError {
            msg: "Invalid rolls format. Must be a positive integer.".to_string(),
        })?;
        // キャプチャグループからダイスの面数（例："6"）を usize にパースする
        let sides = caps[2].parse::<usize>().map_err(|_| MyError {
            msg: "Invalid sides format. Must be a positive integer.".to_string(),
        })?;

        // ロール数と面数が0ではないかチェック（0は不正な値）
        if rolls == 0 || sides == 0 {
            return Err(MyError {
                msg: "Rolls and sides must be greater than 0.".to_string(),
            });
        }
        // ロール数が100を超える、または面数が1000を超える場合はエラーを返す（上限値の検証）
        if rolls > 100 || sides > 1000 {
            return Err(MyError {
                msg: "Rolls must be 100 or fewer, and sides must be 1000 or fewer.".to_string(),
            });
        }

        // 乱数生成器をトップレベルの rng() 関数で取得（rand::thread_rng() の代替）
        let mut rng = rng();
        let mut results = Vec::new();
        // ロール数分、各ダイスの出目を乱数で生成する
        for _ in 0..rolls {
            // 指定された範囲 [1, sides] の乱数を生成（random_range() を利用）
            let roll: usize = rng.random_range(1..=sides);
            results.push(roll);
        }
        // 結果の配列をJSON形式で返す
        Ok(HttpResponse::Ok().json(results))
    } else {
        // 正規表現にマッチしない場合は、フォーマットエラーとしてエラーを返す
        Err(MyError {
            msg: "Invalid format. Use [rolls]d[sides].".to_string(),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数に基づいてenv_loggerを初期化（例: RUST_LOG=info）
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Nginx の log_format main に似たフォーマットを指定
    // 各指定子の意味:
    // %a  -> リモートアドレス
    // %t  -> タイムスタンプ
    // %r  -> リクエストライン（メソッド、パス、HTTPバージョン）
    // %s  -> HTTPステータスコード
    // %b  -> レスポンスボディのバイト数
    // %{Referer}i -> リクエストのRefererヘッダー
    // %{User-Agent}i -> リクエストのUser-Agentヘッダー
    // %{X-Forwarded-For}i -> リクエストのX-Forwarded-Forヘッダー
    // %Dms -> 応答時間（ミリ秒）
    // %{X-Request-ID}i -> リクエストIDヘッダー（任意）
    let log_format = "%a - - [%t] \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" \"%{X-Forwarded-For}i\" %Dms \"%{X-Request-ID}i\"";

    // Actix-web の HTTP サーバーを起動
    HttpServer::new(move || {
        App::new()
            // Logger ミドルウェアを追加して、リクエストごとにログを記録する
            .wrap(Logger::new(log_format))
            // "/roll/{dice}" エンドポイントを定義し、GETリクエストを roll_dice 関数で処理する
            .service(web::resource("/roll/{dice}").route(web::get().to(roll_dice)))
    })
    .bind("0.0.0.0:8080")? // サーバーを localhost のポート8080にバインド
    .run() // サーバーを非同期で実行
    .await
}
