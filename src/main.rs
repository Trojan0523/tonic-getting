/*
 * @Author: BuXiongYu
 * @Date: 2024-10-27 16:04:53
 * @LastEditors: BuXiongYu
 * @LastEditTime: 2024-11-02 21:47:30
 * @Description: 请填写简介
 */
use tonic::transport::Server;
use tonic_getting::{
  grpc::AuthService,
  pb::getting::v1::auth_server::AuthServer
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let grpc_addr = "0.0.0.0:8080".parse()?;
    println!("The gRPC Server listening to {}", grpc_addr);
    Server::builder()
      .add_service(AuthServer::new(AuthService))  // 添加 Auth gRPC Service
      .serve(grpc_addr)
      .await?;

    Ok(())
}
