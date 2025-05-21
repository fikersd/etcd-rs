use ya_etcd_rs::{Client, ClientConfig, Endpoint, KeyValueOp};

#[tokio::main]
async fn main() -> Result<(), Box<ya_etcd_rs::Error>> {
    let cli = Client::new(ClientConfig::new([
        Endpoint::from("https://127.0.0.1:12379")
            .tls(
                "etcd-1",
                "./hack/certs/ca.pem",
                "./hack/certs/etcd-1.pem",
                "./hack/certs/etcd-1-key.pem",
            )
            .await?,
        Endpoint::from("https://127.0.0.1:22379")
            .tls(
                "etcd-2",
                "./hack/certs/ca.pem",
                "./hack/certs/etcd-2.pem",
                "./hack/certs/etcd-2-key.pem",
            )
            .await?,
        Endpoint::from("https://127.0.0.1:32379")
            .tls(
                "etcd-3",
                "./hack/certs/ca.pem",
                "./hack/certs/etcd-3.pem",
                "./hack/certs/etcd-3-key.pem",
            )
            .await?,
    ]))
    .await?;

    cli.put(("foo", "bar")).await.expect("put kv");
    let resp = cli.get("foo").await.expect("get kv");

    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(resp.kvs[0].key_str(), "foo");
    assert_eq!(resp.kvs[0].value_str(), "bar");

    Ok(())
}
