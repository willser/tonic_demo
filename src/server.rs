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