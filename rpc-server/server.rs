use tonic::{transport::Server, Request, Response, Status};
use hello::say_server::{Say, SayServer};
use hello::{SayResponse, SayRequest};

pub mod hello {
  tonic::include_proto!("hello");
}

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
  async fn send(&self, request: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
    println!("Got a request: {:?}", request);
    let reply = hello::SayResponse {
      message: format!("Hello {}!", request.into_inner().name)
    };
    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
  let addr: std::net::SocketAddr = ":50051".parse()?;
  let greeter: MySay = MySay::default();
  Server::builder()
    .add_service(SayServer::new(greeter))
    .serve(addr)
    .await?;
  Ok(())
}
