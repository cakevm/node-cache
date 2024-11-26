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

TODO:
- Implement compression
- Hash keys
- Implement more methods
- Handle latest block

## Usage
Start the node and point your test to the node cache server. When the test is passed stop the server and the recorded requests will be stored in the file.
```shell
node-cache --db-file-path records.db --node http://127.0.0.1:8485 --record
```
Start `anvil` with the `--rpc` flag and point your test to the node cache server.
```shell
anvil --fork-url http://127.0.0.1:7777 --fork-block-number 18567709
```

### GitHub Workflow
In a GitHub Workflow you can download the binary and run it in the background. To save storage compress the record file.
```yaml
      - run: |
          curl -L "https://github.com/cakevm/node-cache/releases/download/v0.0.1/node-cache-v0.0.1-x86_64-unknown-linux-gnu.tar.gz" -o node-cache.tar.xz
          tar -xf node-cache.tar.xz
          chmod +x node-cache
          tar -xf .node-cache/record.tar.gz -C .node-cache
          ./node-cache --db-file-path .node-cache/record.db --node ${{ secrets.MAINNET_HTTP }} &
      - run: MAINNET_HTTP=http://127.0.0.1:7777 make test
```

## Licence
This project is dual-licensed under the Apache 2.0 or MIT licenses. See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.