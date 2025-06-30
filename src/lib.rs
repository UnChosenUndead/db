use conf::Conf;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn connect_to_pg(conf: Conf) -> Result<Pool<Postgres>, sqlx::Error> {
    // DATABASE_URL=postgres://username:password@localhost/database_name
    let database_url_formated = format!(
        "postgres://{}:{}@{}:{}/{}",
        conf.pg_db_username, conf.pg_db_password, conf.pg_db_host, conf.pg_db_port, conf.pg_db_name,
    );
    println!("{}", database_url_formated);
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Set the maximum number of connections
        .connect(&database_url_formated)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Deserialize, Serialize, PartialEq, sqlx::FromRow)]
    struct TestTable {
        id: i32,
        name: String,
    }

    fn init_test_table() -> TestTable {
        TestTable {
            id: 1,
            name: "test".to_string(),
        }
    }

    fn init_test_conf() -> Conf {
        Conf {
            pg_db_port: "15444".to_string(),
            pg_db_host: "127.0.0.1".to_string(),
            pg_db_name: "postgres".to_string(),
            pg_db_username: "postgres".to_string(),
            pg_db_password: "postgres".to_string(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_connect_to_pg() {
        let x = connect_to_pg(init_test_conf()).await;
        let mut i = false;
        match x {
            Ok(_) => i = true,
            Err(_) => i = false,
        }
        assert!(i, "connection")
    }

    #[tokio::test]
    async fn test_connect_to_pg_and_get_some_data_from_table() {
        let pg_pool = connect_to_pg(init_test_conf()).await.unwrap();
        let first_test_query = sqlx::query_as!(
            TestTable,
            r#"
                SELECT id, name
                FROM test
                WHERE id = 1
                ORDER BY id
            "#,
        )
        .fetch_one(&pg_pool)
        .await
        .unwrap();

        assert!(first_test_query == init_test_table(), "table test")
    }
}
