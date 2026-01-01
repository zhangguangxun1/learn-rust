#[cfg(test)]
mod tests {
    use sqlx::postgres::PgPoolOptions;
    use chrono::{DateTime, Utc};

    #[tokio::test]
    async fn test_query_now_with_sqlx() {
        // 1. 准备连接字符串 (确保用户 tiku_rw 已被授予访问 open_tiku 数据库的权限)
        let db_url = "postgres://tiku_rw:123456@localhost/open_tiku";

        // 2. 创建连接池
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(db_url)
            .await
            .expect("无法连接到 PostgreSQL 数据库，请检查用户名、密码或数据库是否存在");

        // 3. 执行查询 SELECT NOW()
        // sqlx 的 query_as 可以直接将结果映射到类型
        let row: (DateTime<Utc>,) = sqlx::query_as("SELECT NOW()")
            .fetch_one(&pool)
            .await
            .expect("查询执行失败");

        println!("数据库当前时间: {}", row.0);

        // 4. 断言验证
        assert!(row.0 <= Utc::now(), "数据库时间不应晚于当前系统时间");
    }
}
