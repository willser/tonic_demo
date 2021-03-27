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