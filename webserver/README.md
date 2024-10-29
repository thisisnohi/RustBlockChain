# blockchain

> create by nohi

## 功能

### 区块功能

> /blockchain

* 新增区块：  `PUT` `/blockchain/`
* 获取最新区块 `GET` `/blockchain/last-block`
* 列出所有区块 `GET` `/blockchain/`

### 节点功能

> nodes

* 新增节点
* 列出所有节点
* 广播所有节点
* 其他节点广播新增区块

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