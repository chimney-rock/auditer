use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use futures::{future, Future, Stream};
use tower_hyper::server::{Http, Server};

use crate::APP_SETTINGS;

pub mod generated {
  include!(concat!(env!("OUT_DIR"), "/auditing.rs"));
}

use generated::{server, HelloReply, HelloRequest};

#[derive(Clone, Debug)]
struct Greet;

impl server::Auditer for Greet {
  type SayHelloFuture = future::FutureResult<Response<HelloReply>, tower_grpc::Status>;

  fn say_hello(&mut self, request: Request<HelloRequest>) -> Self::SayHelloFuture {
    println!("REQUEST = {:?}", request);

    let response = Response::new(HelloReply {
      message: "Zomg, it works!".to_string(),
    });

    future::ok(response)
  }
}

pub fn start_server() -> Result<(), failure::Error> {
  let new_service = server::AuditerServer::new(Greet);
  let mut server  = Server::new(new_service);
  let http        = Http::new().http2_only(true).clone();

  let bind = TcpListener::bind(&APP_SETTINGS.inbound_listener.address)?;

  let serve = bind
    .incoming()
    .for_each(move |socket| {
      if let Err(err) = socket.set_nodelay(true) {
        return Err(err);
      }

      let serve = server.serve_with(socket, http.clone());
      tokio::spawn(serve.map_err(|err| error!("Hyper error: {:?}", err)));

      Ok(())
    })
    .map_err(|err| warn!("GRPC accept error: {}", err));

  tokio::run(serve);
  Ok(())
}
