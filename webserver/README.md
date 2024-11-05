# blockchain

> create by nohi

## 功能

### 区块功能

> /blockchain

* 新增区块：  `PUT` `/blockchain/`
* 获取最新区块 `GET` `/blockchain/last-block`
* 列出所有区块 `GET` `/blockchain/`
* 广播所有节点(客户端)  `OPTION /blockchain/broadcast`
* 接收其他节点广播的新增区块 `OPTION /blockchain/broadcast`

### 节点功能

> nodes

* 新增节点
* 列出所有节点

## 测试

* 初始化
  启动actix_web时，会自动初始化区块链，并把链放入共享内存中。
  后续可把共享内存中的链，物理化保存，启动时再读取物理化保存的链数据。

* 获取区块列表
     ```http request
      http://127.0.0.1:3000/blockchain/
     ```

* 添加数据至链中
    ```http request
    PUT http://127.0.0.1:3000/blockchain/
    Content-Type: application/json
    
    {
      "data": "some data"
    }
    ```
* 列出所有节点
  `GET http://{{local}}/nodes`

* 连接指定节点
  ```http request
  PUT http://{{local}}/nodes
  content-type: application/json
  
  {
    "node": "http://127.0.0.1:4000"
  }
  ```

## 问题与解决

* 链及节点使用共享内存，可能导致并发问题
    * 如：广播时，获取节点锁、块，循环节点进行广播。这里改成异步任务方式进行。
      `block_chain_service.rs Line 108`
* 异步任务出现，值移动问题。使用外部变更，在异步任务中进行值clone
  `block_chain_service.rs  Line 113 解决 broad_block 值移动问题`