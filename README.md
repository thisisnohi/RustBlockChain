# README

> create by nohi 20241011

## 过程

### 区块定义

* data： 数据
* prev_block_hash: 上一区域hash
* hash: data + prev_block_hash得到的本区块Hash

### 区块链范式

矿机不断循环的调整随机数来得出正确的哈希值

## 需要解决的问题

* 创世块
* 创建新区块
* 验证块的合法性
* 保存区块链
* 验证区块链完整性、合法性
* 选择最长链
* 节点间通讯
    * 运行服务（模拟一个主服务，当注册中心）
    * 主节点：注册中心
    * 各节点：客户端，向注册中心注册节点，并拉取其他节点列表

### 暂无法解决的问题

* 分叉
* 超半数验证

## 工程

### webserver

* 运行`cargo run --bin block-chain`
  ```shell
  cargo run --bin block-chain
  -- 自动重载
  cargo install cargo-watch
  cargo watch -x 'run --bin block-chain'
  ```