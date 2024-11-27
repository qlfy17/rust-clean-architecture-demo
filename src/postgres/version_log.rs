use super::DB;

pub struct VersionLog {
    pub source: String,
    pub created_at: i64,
}

impl DB {
    pub async fn latest_proxy_index_update(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_as(
            "SELECT created_at
		FROM version_logs
		WHERE source={}
		ORDER BY created_at DESC
		LIMIT 1",
        )
        .bind("proxy-index")
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }

    pub async fn insert_version_logs(&self, logs: Vec<VersionLog>) -> Result<(), sqlx::Error> {
        return self.transact(|tx| async move {
            for log in logs {
                sqlx::query(
                    "INSERT INTO version_logs (source, created_at)
			VALUES ($1, $2)",
                )
                .bind(log.source)
                .bind(log.created_at)
                .execute(&tx)
                .await?;
            }
        });
    }
}
