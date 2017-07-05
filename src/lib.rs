extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate grpc;
extern crate tls_api;

pub mod auth;
pub mod kv;
pub mod rpc;
pub mod rpc_grpc;

pub use self::auth::*;
pub use self::kv::*;
pub use self::rpc::*;
pub use self::rpc_grpc::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
