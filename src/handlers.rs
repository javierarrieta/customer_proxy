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
use futures::future;


#[derive(Clone)]
pub struct CustomerHandler { pub customer: Customer }

impl NewHandler for CustomerHandler {
    type Instance = Self;
    
    fn new_handler(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl Handler for CustomerHandler {

    fn handle(self, state: State) -> Box<HandlerFuture> {
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((
                serde_json::to_vec(&self.customer).expect("serialized product"),
                mime::APPLICATION_JSON,
            )),
        );
        Box::new(future::ok((state, res)))
    }
}