use ya_etcd_rs::{AuthOp, AuthRoleAddRequest, AuthRoleDeleteRequest, Client, ClientConfig};

async fn role(cli: &Client) -> Result<(), Box<ya_etcd_rs::Error>> {
    cli.role_add(AuthRoleAddRequest::new("foo_role"))
        .await
        .expect("role add");

    let resp = cli.role_list().await.expect("role list");

    assert!(!resp.roles.is_empty());

    cli.role_delete(AuthRoleDeleteRequest::new("foo_role"))
        .await
        .expect("role delete");

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
    let resp = cli.auth_status().await?;
    if resp.enabled {
        println!("auth is enabled...");
        cli.auth_disable().await?;
        let resp = cli.auth_status().await?;
        assert!(!resp.enabled);
        cli.auth_enable().await?;
        let resp = cli.auth_status().await?;
        assert!(resp.enabled);
    }
    role(&cli).await?;

    Ok(())
}
