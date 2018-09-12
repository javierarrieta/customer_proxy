extern crate gotham;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate mime;
extern crate futures;

use std::io;

use model::Customer;

use gotham::http::response::create_response;
use gotham::handler::{Handler, NewHandler, HandlerFuture};
use gotham::state::State;
use hyper::StatusCode;
use futures::{future, Future};

use redis_client::CustomerClient;

#[derive(Clone)]
pub struct CustomerHandler { client : CustomerClient }

impl NewHandler for CustomerHandler {
    type Instance = Self;
    
    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for CustomerHandler {

    fn handle(self, state: State) -> Box<HandlerFuture> {
        
        let customer_f = Future<Item = Customer, Error = String> = 
            self.client.get_customer("1").and_then(|customer|
        
        );

        // let res = create_response(
        //     &state,
        //     StatusCode::Ok,
        //     Some((
        //         serde_json::to_vec(&self.customer).expect("serialized product"),
        //         mime::APPLICATION_JSON,
        //     )),
        // );
        // Box::new(future::ok((state, res)))
    }
}