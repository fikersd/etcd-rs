use std::time::Duration;

use tokio::time::sleep;
use ya_etcd_rs::{Client, ClientConfig, KeyValueOp};

async fn put_with_sleep(cli: &Client, secs: u64) -> Result<(), Box<ya_etcd_rs::Error>> {
    cli.put(("foo", "bar")).await.expect("put kv");
    let resp = cli.get("foo").await.expect("get kv");

    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(resp.kvs[0].key_str(), "foo");
    assert_eq!(resp.kvs[0].value_str(), "bar");
    // Sleep until the current token expires
    sleep(Duration::from_secs(secs)).await;

    let resp = cli.delete("foo").await.expect("delete kv");
    assert_eq!(resp.deleted, 1);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<ya_etcd_rs::Error>> {
    let cfg = ClientConfig::new(
        [
            "http://127.0.0.1:12379".into(),
            "http://127.0.0.1:22379".into(),
            "http://127.0.0.1:32379".into(),
        ]
        .to_vec(),
    )
    .auth("".to_owned(), "".to_owned());

    let cli = Client::new(cfg).await?;
    // The default expiration time for etcd authentication tokens is 10 minutes
    put_with_sleep(&cli, 601).await?;
    Ok(())
}
