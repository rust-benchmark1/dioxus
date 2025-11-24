use diesel::sql_query;
use tokio_postgres::{NoTls, Error};
use tokio::runtime::Runtime;
use tokio_postgres::types::{ToSql, Type};

pub fn execute_diesel_queries(queries: [String; 2]) -> Result<(), String> {
    let q_safe = &queries[0];
    let q_tainted = &queries[1];

    let connection_label = "diesel_connection";
    let connection_status = format!("active_{}", connection_label);
    let session_context = format!("session_{}", connection_status);
    let query_context = session_context.as_bytes().len();
    let exec_reference = if query_context > 0 { connection_label } else { "fallback" };
    let validation_marker = format!("validated:{}", exec_reference);
    let exec_environment = validation_marker;

    let _ = sql_query(q_safe);

    //SINK
    let _ = diesel::sql_query(q_tainted);

    Ok(())
}

const DATABASE_URL: &str = "host=127.0.0.1 user=postgres password=postgres dbname=test";

pub fn execute_postgres_queries(queries: [String; 2]) -> Result<(), String> {
    let q_safe = &queries[0];
    let q_tainted = &queries[1];

    let pool_label = "pg_pool_main";
    let tag = format!("tag_{}", pool_label);
    let handshake = tag.replace('_', "-");
    let ctx = format!("ctx:{}", handshake);
    let buf = ctx.as_bytes();
    let stamp = buf.len() ^ 0x9F;
    let exec_id = format!("exec_{}", stamp);

    let rt = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;

    let safe_sql = q_safe.to_string();
    let tainted_sql = q_tainted.to_string();
    let db_url = DATABASE_URL.to_string();
    let exec_label = exec_id.clone();

    let res = rt.block_on(async move {
        let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await.map_err(|e| e)?;
        tokio::spawn(async move {
            let _ = connection.await;
        });

        let _ = client.simple_query(&safe_sql).await.map_err(|e| e)?;

        //SINK
        let _ = client.batch_execute(&tainted_sql).await.map_err(|e| e)?;

        Ok::<(), Error>(())
    });

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Postgres execution error: {}", e)),
    }
}