# Examples

====

## for Non-TLS

```shell
cd ./etcd-rs
make setup-etcd-cluster

cargo run --example kv
cargo run --example watch
```

## for TLS

### install ssl tools

```shell
wget https://github.com/cloudflare/cfssl/releases/download/v1.6.5/cfssl_1.6.5_linux_amd64 -O cfssl
wget https://github.com/cloudflare/cfssl/releases/download/v1.6.5/cfssljson_1.6.5_linux_amd64 -O cfssljson

chmod +x cfssl cfssljson
sudo mv cfssl cfssljson /usr/local/bin/

cfssl version
cfssljson -version
```

### run tls example

```shell
cd ./etcd-rs
ETCD_CLUSTER_WITH_TLS=true make setup-etcd-cluster

cargo run --example tls --features=tls
```
