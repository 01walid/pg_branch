pub mod git;

use std::{fmt::Debug, time::Duration};

use color_eyre::eyre::Result;
use eyre::eyre;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use url::Url;

#[derive(Debug)]
pub struct PostgresManager {
    _host: String,
    _username: String,
    _password: String,
    _port: u16,
    _db_name: String,
    _conn_params: String,
    pool: Pool<Postgres>,
}

impl PostgresManager {
    pub async fn new(conn_string: &str) -> Result<PostgresManager> {
        let parsed_conn_str = Self::parse_conn_string(conn_string).await?;

        let pool = PgPoolOptions::new()
            .connect_timeout(Duration::new(3, 0))
            .connect(parsed_conn_str.as_str())
            .await?;

        Ok(PostgresManager {
            _host: parsed_conn_str.host_str().unwrap_or("").into(),
            _username: parsed_conn_str.username().into(),
            _password: parsed_conn_str.password().unwrap_or("").into(),
            _port: parsed_conn_str.port().unwrap_or(5432),
            _db_name: parsed_conn_str
                .path()
                .strip_prefix('/')
                .unwrap_or("")
                .into(),
            _conn_params: parsed_conn_str.query().unwrap_or("").into(),
            pool,
        })
    }

    pub async fn drop_db(&self, db_name: &str) -> Result<()> {
        let result = sqlx::query("DROP DATABASE $1")
            .bind(db_name)
            .execute(&self.pool)
            .await?;

        println!("{:?}", result.rows_affected());

        Ok(())
    }

    async fn parse_conn_string(conn_str: &str) -> Result<Url> {
        let parsed = Url::parse(conn_str)?;
        if ["postgres", "postgresql"].contains(&parsed.scheme()) {
            eyre!("Only postgres:// and postgresql:// are supported in the connection string");
        }
        Ok(Url::parse(conn_str)?)
    }

    pub async fn ping_db(&self) -> Result<()> {
        let _: (i64,) = sqlx::query_as("SELECT $1")
            .bind(1_i64)
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn mark_db_as_template(&self, db_name: &str) -> Result<()> {
        let q = format!("ALTER DATABASE \"{db_name}\" WITH is_template TRUE");
        let result = sqlx::query(q.as_str()).execute(&self.pool).await?;

        println!("{:?}", result.rows_affected());
        println!("Database marked as template!");

        Ok(())
    }

    pub async fn fork_db(
        &self,
        template_db_name: &str,
        target_db_name: &str,
        rename: bool,
    ) -> Result<()> {
        if self.db_exists(target_db_name).await? {
            println!("Db already exists, skipping the fork!");
            return Ok(());
        }
        self.kill_db_conn(template_db_name).await?;
        let q = format!("CREATE DATABASE \"{target_db_name}\" TEMPLATE \"{template_db_name}\"");
        let result = sqlx::query(q.as_str()).execute(&self.pool).await?;

        println!("{:?}", result.rows_affected());

        Ok(())
    }

    pub async fn kill_db_conn(&self, target_db: &str) -> Result<()> {
        let q = format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
             WHERE pg_stat_activity.datname = '{target_db}'
               AND pid <> pg_backend_pid();"
        );
        let result = sqlx::query(q.as_str()).execute(&self.pool).await?;
        println!("{:?}", result.rows_affected());
        Ok(())
    }

    pub async fn db_exists(&self, target_db: &str) -> Result<bool> {
        let q = format!("SELECT 1 FROM pg_database WHERE datname='{target_db}'");
        let res = sqlx::query(q.as_str()).execute(&self.pool).await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn rename_db(&self, orig_name: &str, new_name: &str) -> Result<()> {
        let q = format!("ALTER DATABASE \"{orig_name}\" RENAME TO \"{new_name}\"");
        sqlx::query(q.as_str()).execute(&self.pool).await?;
        Ok(())
    }
}
