#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate redis_async;

extern crate gotham;
extern crate hyper;
extern crate futures;

mod model;
mod handlers;
mod router;
mod redis_client;

use futures::Future;
use redis_async::client;

use redis_client::RedisCustomerClient;
use handlers::CustomerHandler;

fn main() {

    let redis_addr = "127.0.0.1:6379".to_owned().parse().unwrap();
    let redis_client = client::paired_connect(&redis_addr).wait().unwrap();
    let customer_client = RedisCustomerClient::new(redis_client);
    let customer_handler = CustomerHandler::new(&customer_client);

    let addr = "0.0.0.0:9000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router::customer_router(customer_handler));
}