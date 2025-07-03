use anyhow::Result;
use scylla::client::session_builder::SessionBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    println!("Connecting to {uri} with cassandra superuser ...");

    let session = SessionBuilder::new()
        .known_node(uri)
        .user("cassandra", "cassandra")
        .build()
        .await
        .unwrap();

    session.query_unpaged("CREATE KEYSPACE IF NOT EXISTS examples_ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await.unwrap();
    session
        .query_unpaged("DROP TABLE IF EXISTS examples_ks.auth;", &[])
        .await
        .unwrap();

    println!("Ok.");

    Ok(())
}
