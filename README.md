# yet another etcd client for Rust

[<img alt="github" height="20" src="https://img.shields.io/badge/github-lodrem/etcd--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/Fiekers/etcd-rs)
[<img alt="crates.io" height="20" src="https://img.shields.io/crates/v/etcd-rs.svg?style=for-the-badge&color=fc8d62&logo=rust">](https://crates.io/crates/ya-etcd-rs)
[<img alt="docs.rs" height="20" src="https://img.shields.io/badge/docs.rs-etcd--rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white">](https://docs.rs/ya-etcd-rs)
[<img alt="build status" height="20" src="https://img.shields.io/github/actions/workflow/status/Fiekers/etcd-rs/ci.yml?branch=master&style=for-the-badge">](https://github.com/Fiekers/etcd-rs/actions?query%3Amaster)
[<img alt="dependency status" height="20" src="https://deps.rs/repo/github/Fiekers/etcd-rs/status.svg?style=for-the-badge">](https://deps.rs/repo/github/Fiekers/etcd-rs)

An [etcd](https://github.com/etcd-io/etcd) (API v3) client for Rust backed by [tokio](https://github.com/tokio-rs/tokio) and [tonic](https://github.com/hyperium/tonic).

## Supported APIs

- KV
  - [x] Put
  - [x] Range
  - [x] Delete
  - [x] Transaction
  - [x] Compact
- Lease
  - [x] Grant
  - [x] Revoke
  - [x] KeepAlive
  - [x] TimeToLive
- Watch
  - [x] WatchCreate
  - [x] WatchCancel
- Auth
  - [x] Authenticate
  - [x] AuthStatus
  - [x] AuthEnable
  - [x] AuthDisable
  - [x] RoleAdd
  - [x] RoleDelete
  - [x] RoleList
  - [ ] RoleGrantPermission
  - [ ] UserAdd
  - [ ] UserDelete
  - [ ] UserGrantRole
- Cluster
  - [x] MemberAdd
  - [x] MemberRemove
  - [x] MemberUpdate
  - [x] MemberList
- Maintenance
  - [ ] Alarm
  - [ ] Status
  - [ ] Defragment
  - [ ] Hash
  - [ ] Snapshot
  - [ ] MoveLeader

### Rust Version

`ya-etcd-rs`'s MSRV is `1.75`.

```bash
rustup update
cargo build
```

## Usage

Add following dependencies in your project `cargo.toml`:

```toml
[dependencies]
ya-etcd-rs = "1.2"
```

```rust
use ya_etcd_rs::{Client, ClientConfig, KeyValueOp, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Client::new(ClientConfig::new([
        "http://127.0.0.1:12379".into(),
        "http://127.0.0.1:22379".into(),
        "http://127.0.0.1:32379".into(),
    ]))
    .await?;

    cli.put(("foo", "bar")).await.expect("put kv");

    let kvs = cli.get("foo").await.expect("get kv").kvs;
    assert_eq!(kvs.len(), 1);
    Ok(())
}
```

## Development

requirements:

- Makefile
- docker
- docker compose

## Start local etcd cluster

```shell
make setup-etcd-cluster
```

stop cluster

```shell
make teardown-etcd-cluster
```

for specified case:

```shell
TEST_CASE=test_put_error make test-one
```

## License

This project is licensed under the [MIT license](LICENSE).
