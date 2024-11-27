use sqlx::{PgPool, Transaction};
use std::{future::Future, pin::Pin};

/// 数据库封装结构
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    /// 打开数据库连接
    pub async fn open(dbinfo: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(dbinfo).await?;
        Ok(Self { pool })
    }

    /// 在事务中执行逻辑
    pub async fn transact<F, T>(&self, tx_func: F) -> Result<T, sqlx::Error>
    where
        F: FnOnce(
            &mut Transaction<'_, sqlx::Postgres>,
        ) -> Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + Send>>,
    {
        let mut tx = self.pool.begin().await?;

        // 使用事务函数执行操作
        let result = tx_func(&mut tx).await;

        // 根据事务结果决定是提交还是回滚
        match result {
            Ok(value) => {
                tx.commit().await?;
                Ok(value)
            }
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }
}
