use std::{net::ToSocketAddrs, pin::Pin, time::Duration};

use futures::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod echo {
    tonic::include_proto!("grpc.examples.echo");
}

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<echo::EchoResponse, Status>> + Send>>;

struct Service;

#[tonic::async_trait]
impl echo::echo_server::Echo for Service {
    type EchoStream = ResponseStream;

    async fn echo(&self, _req: Request<echo::EchoRequest>) -> EchoResult<Self::EchoStream> {
        // CPU work
        std::thread::sleep(Duration::from_secs(10_000));

        unreachable!()
    }
}

async fn main_server() {
    let server = Service {};
    Server::builder()
        .add_service(echo::echo_server::EchoServer::new(server))
        .serve("[::1]:8083".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
}

async fn main_client() {
    let endpoint = tonic::transport::Endpoint::try_from("http://[::1]:8083").unwrap().http2_keep_alive_interval(Duration::from_millis(1_000));
    let mut client = echo::echo_client::EchoClient::connect(endpoint)
        .await
        .unwrap();

    let mut stream = client
        .echo(echo::EchoRequest {
            message: "foo".into(),
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(item) = stream.next().await {
        println!("main_client: received: {}", item.unwrap().message);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let server = tokio::spawn(main_server());
    let client = tokio::spawn(main_client());
    client.await.unwrap();
    server.abort();
    let err = server.await.unwrap_err();
    assert!(err.is_cancelled());
}
