use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};

pub fn read_certs() -> SslAcceptorBuilder {
  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
  builder.set_private_key_file("certs/key.pem", SslFiletype::PEM).expect("No private key file found");
  builder.set_certificate_chain_file("certs/cert.pem").expect("No certificate found");
  
  builder
}