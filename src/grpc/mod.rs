use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use futures::{future, Future, Stream};
use tower_hyper::server::{Http, Server};

use crate::APP_SETTINGS;

pub mod generated {
  include!(concat!(env!("OUT_DIR"), "/auditing.rs"));
}

use generated::{server, ObjectCreateRequest, ObjectUpdateRequest, Empty};

#[derive(Clone, Debug)]
struct Greet;

impl server::Auditer for Greet {
  type ObjectCreateFuture = future::FutureResult<Response<Empty>, tower_grpc::Status>;
  fn object_create(&mut self, request: Request<ObjectCreateRequest>) -> Self::ObjectCreateFuture {
    println!("REQUEST = {:?}", request);

    let response = Response::new(Empty {
      temp: "Zomg, it works!".to_string(),
    });

    future::ok(response)
  }

  type ObjectUpdateFuture = future::FutureResult<Response<Empty>, tower_grpc::Status>;
  fn object_update(&mut self, request: Request<ObjectUpdateRequest>) -> Self::ObjectUpdateFuture {
    println!("REQUEST = {:?}", request);

    let response = Response::new(Empty {
      temp: "Zomg, it works!".to_string(),
    });

    future::ok(response)
  }
}

pub fn build() -> impl Future<Item = (), Error = ()> + Send {
  let new_service = server::AuditerServer::new(Greet);
  let mut server  = Server::new(new_service);
  let http        = Http::new().http2_only(true).clone();

  TcpListener::bind(&APP_SETTINGS.grpc_listener.address)
    .expect("Unable to bind gRPC")
    .incoming()
    .for_each(move |socket| {
      if let Err(err) = socket.set_nodelay(true) {
        return Err(err);
      }

      let serve = server.serve_with(socket, http.clone());
      tokio::spawn(serve.map_err(|err| error!("Hyper error: {:?}", err)));

      Ok(())
    })
    .map_err(|err| warn!("GRPC accept error: {}", err))
}
