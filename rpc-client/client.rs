use hello::say_client::SayClient;
use hello::SayRequest;

pub mod hello {
  tonic::include_proto!("hello");
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
  let mut client: SayClient<tonic::transport::Channel> = SayClient::connect("http://[::1]:50051").await?;
  let request: tonic::Request<SayRequest> = tonic::Request::new(SayRequest {
    name: "Gin and Tonic".into()
  });
  let response: tonic::Response<hello::SayResponse> = client.send(request).await?;

  println!("Response = {}", response.into_inner().message);

  Ok(())
}
