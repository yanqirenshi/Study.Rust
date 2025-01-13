use sqlx::{mysql::MySqlPool, Row};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // DATABASE_URLを取得
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // MySQL用の接続プールを作成
    let pool = MySqlPool::connect(&database_url).await?;

    // サンプルクエリ: usersテーブルのデータを取得
    let rows = sqlx::query("SELECT id, name, email FROM user")
        .fetch_all(&pool)
        .await?;

    // 結果を表示
    for row in rows {
        let id: i32 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let email: String = row.try_get("email")?;
        println!("ID: {}, Name: {}, Email: {}", id, name, email);
    }

    Ok(())
}
