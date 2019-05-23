use openssl::ssl::{SslMethod, SslAcceptor, SslAcceptorBuilder, SslFiletype};
use actix_web::{middleware::{Logger, cors::Cors}, App, HttpServer, web};
use failure::Fallible;
use std::io;


pub struct Server {
  pub sys: actix_rt::SystemRunner
}

impl Server {
  /// Creates a new instance of the HTTP server.
  pub fn new() -> Fallible<Self> {
    let sys = actix_rt::System::new("auditor");

    let server = HttpServer::new(move || {
      App::new()

    })
  }

  /// Creates an SSL Acceptor object.
  /// 
  /// # Arguments
  /// * `tls` - TLS configuration settings.
  fn build_tls(private_key: &str, cert: &str) -> Fallible<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(private_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(cert)?;
    Ok(builder)
  }
}
