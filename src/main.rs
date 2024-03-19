use sqlx::Row;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:1234@localhost:5432/users_crud";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("SELECT 1 + 1 sum").fetch_one(&pool).await?;

    let sum: i32 = res.get("sum");

    println!("1 + 1 = {}", sum);
    Ok(())
}
