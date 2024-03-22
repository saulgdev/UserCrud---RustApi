use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

struct User {
    id: uuid::Uuid,
    name: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    is_adm: bool,
}

async fn create_user(user: &User, database: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO users (id, name, email, password, created_at, updated_at, is_adm) VALUES ($1, $2, $3, $4, $5, $6, $7)";

    sqlx::query(query)
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(user.is_adm)
        .execute(database)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:1234@localhost:5432/users_crud";

    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let user = User {
        id: Uuid::new_v4(),
        name: "Saul Gomes Sousa".to_string(),
        email: "saul.contatodev@gmail.com".to_string(),
        password: "senhadificil".to_string(),
        is_adm: true,
        created_at: Utc::now(),
        updated_at: None,
    };

    create_user(&user, &pool).await?;

    Ok(())
}
