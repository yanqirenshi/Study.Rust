use sqlx::mysql::MySqlPool;
use sqlx::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // DATABASE_URLを環境変数から取得
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // MySQL用の接続プールを作成
    let pool = MySqlPool::connect(&database_url).await?;

    // サンプルクエリ: テーブル `users` からデータを取得
    let rows = sqlx::query!("SELECT id, name, email FROM iwo_studio.user;")
        .fetch_all(&pool)
        .await?;

    // 結果を表示
    for row in rows {
        println!(
            "ID: {}, Name: {}, Email: {}",
            row.id,
            row.name,
            row.email
        );
    }

    Ok(())
}
