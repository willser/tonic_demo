# 使用tonic进行grpc的开发

## 前言
- 如果您使用`idea`开发，可以下载idea的[proto插件](https://github.com/ksprojects/protobuf-jetbrains-plugin/releases)，这样可以获得更好的proto文件编辑体验 :smile:。

## 新项目
首先通过`cargo new`或者其他方式创建一个新的`rust`项目。

其次在`Cargo.toml`中加入以下配置

````TOML
[dependencies]
# protoBuf
prost = "0.7"
# grpc
tonic = {version="0.4.1",features = ["tls"]}
# tokio add "rt-multi-thread" features to fix error: The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.
tokio = {version="1.0",features = ["macros","rt-multi-thread"]}
# aysnc
futures = "0.3"

[build-dependencies]
# build create *.rs
tonic-build = "0.4.1"
````

## 编写文件
在项目根目录下创建文件`proto/hello_world.proto`，编辑其加入以下内容
````proto

syntax = "proto3";

package hello_world;

service HelloWord {

    rpc Send (HelloWordRequest) returns (HelloWordResponse);
}


message HelloWordRequest {

    string name = 1;
}

message HelloWordResponse {

    string message = 1;
}
````
在项目根目录下创建文件`build.rs`，编辑其加入以下内容
````rs
fn main()->Result<(),Box<dyn std::error::Error>>{
    // compiling protos using path on build time
    tonic_build::compile_protos("proto/hello_world.proto")?;
    Ok(())
}
````
第一个文件的是进行grpc服务的定义，第二个文件用于辅助生成服务相关代码。

此时项目的结构为
```
tonic_demo
  |— proto
  |        └─ hello_world.proto
  |— src
  |      └─ main.rs
  |— build.rs
 └─  Cargo.toml
```

## 获取模块文件
*由于在ild-dependencies` 中加入了`tonic-build = "0.4.1"`，所以在执行`cargo build`时会自动调用`build.rs`中的代码，由于我们在`build.rs`添加了`proto/hello_world.proto`，因此会根据这个proto文件生成对应的代码*
执行`cargo build`
执行成功的话在这个路径下```/target/debug/build/<your_project_name>-<uuid>``` 下会找到一个```out```文件夹，其中包含着生成的hello_world.rs
比如我的是```` target/debug/build/tonic_demo-d2b2af6c99ff77b8 ````

*如果修改了proto文件，需要同步改掉.rs文件的话需要删掉该路径或者执行````cargo clean````之后重新build*

将hello_world.rs复制到src文件夹下，并在main.rs中定义````mod hello_world````

## 创建客户端与服务
在Cargo.toml中增加如下代码，定义一个server和一个client。
```toml

[[bin]]
name="hello_worl_server"
path="src/server.rs"

[[bin]]
name="hello_worl_cilent"
path="src/cilent.rs"
```
在对应路径创建rs文件，编辑server.rs
````rs
use tonic::transport::Server;
use hello_world::hello_word_server::{HelloWord, HelloWordServer};
use hello_world::{HelloWordRequest, HelloWordResponse};
use tonic::{Response, Status, Request};

mod hello_world;

#[derive(Default)]
struct ServerMsg {}

#[tonic::async_trait]
impl HelloWord for ServerMsg {
    async fn send(&self, request: Request<HelloWordRequest>) -> Result<Response<HelloWordResponse>, Status> {
        Ok(Response::new(HelloWordResponse { message: format!("get msg =  {{ {} }}", request.into_inner().name) }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建监听ip端口
    let addr = "0.0.0.0:6789".parse().unwrap();
    // 创建服务
    let say = ServerMsg::default();
    // 创建服务
    Server::builder()
        .add_service(HelloWordServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
````

编辑client.rs
````rs
use hello_world::hello_word_client::HelloWordClient;
use hello_world::{HelloWordRequest};

mod hello_world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接服务 创建channel
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:6789")
        .connect()
        .await?;
    // 通过channel创建客户端
    let mut client = HelloWordClient::new(channel);
    // creating a new Request
    let request = tonic::Request::new(
        HelloWordRequest {
            name: String::from("test")
        },
    );
    // 发送请求
    let response = client.send(request).await?.into_inner();
    // 打印返回值
    println!("返回值为 {:?}", response);
    Ok(())
}
````

## 测试
依次运行`server.rs`与`client.rs`，可以在`client.rs`的运行窗口获取以下输出
````
返回值为 HelloWordResponse { message: "get msg =  { test }" }
````
由此，tonic最基础的功能就完成了。
