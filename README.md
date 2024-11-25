# Node cache
When your tests are using Ethereum node to fetch the storage slots by json-rpc it can slow down the test execution. To avoid this, you can cache the storage slots in a file and use it in the next test runs. 

Node cache will store all rpc requests in a file that is compressed and can be stored in the repository.

## Concept
Cache everything that is deterministic and you get for the same request the same result from a node.

```
TEST -> ANVIL -> NODE CACHE -> REAL NODE
```

## Status
Currently, node cache is in a pre-alpha state.

## Usage
Start the node and point your test to the node cache server. When the test is passed stop the server and the recorded requests will be stored in the file.
```shell
node-cache --db-file-path records.db --node http://127.0.0.1:8485 --record
```
Start `anvil` with the `--rpc` flag and point your test to the node cache server.
```shell
anvil --fork-url http://127.0.0.1:7777 --fork-block-number 18567709
```

## Licence
This project is dual-licensed under the Apache 2.0 or MIT licenses. See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.