# Raft KVS Server (from n月刊ラムダノート Vol.5,No.1 2025 #3 実用Raft)

[n月刊ラムダノート Vol.5, No.1(Mar.2025)](https://www.lambdanote.com/products/n-vol-5-no-1) に収録されている「実用Raft」の学習リポジトリ

## Getting Started

### server

```sh
# run kvs cluster
$ cargo install --path .
$ raftkvs 6000
$ jlot req CreateCluster | jlot call :6000

# add server
$ raftkvs 7000
$ jlot req AddServer '{"addr":"127.0.0.1:7000"}' | jlot call :6000
```

### client

```sh
# send request with jlot
$ cargo install jlot
$ jlot req Apply '{"input":{"Put":{"key":"foo","value":123}},"kind":"Command"}' | jlot call :6000

# random json data
$ cargo install rjg
$ rjg --count 100000 \
--var key='{"$str":["$alpha", "$alpha", "$alpha"]}' \
--var put='{"Put":{"key":"$key", "value":"$u32"}}' \
--var get='{"Get":{"key":"$key"}}' \
--var delete='{"Delete":{"key":"$key"}}' \
'{"jsonrpc":"2.0", "id": "$i", "method":"Apply", "params":{"input":{"$oneof":["$put","$get","$delete"]},"kind":"Command"}}' > commands.jsonl

# call with 1 concurrency
$ cat commands.jsonl | jlot call :6000 -a -c 1 | jlot stats | jq .
```
