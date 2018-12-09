extern crate gotham;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate mime;
extern crate futures;

use model::Customer;

use std::io;
use gotham::http::response::create_response;
use gotham::handler::{Handler, HandlerFuture, HandlerError, IntoHandlerError};
use gotham::state::State;
use hyper::{StatusCode, Response};

use futures::prelude::*;
use futures::Future;

use redis_client::CustomerClient;

#[derive(Clone)]
pub struct CustomerHandler<'a> { client : &'a CustomerClient }

impl<'a> CustomerHandler<'a> {

    pub fn new(client: &'a CustomerClient) -> CustomerHandler<'a> {
        CustomerHandler { client: client }
    }
}

fn create_customer_response(state: &State, customer: &Customer) -> Response {
    create_response(
        state,
        StatusCode::Ok,
        Some((
            serde_json::to_vec(customer).expect("serialized product"),
            mime::APPLICATION_JSON,
        ))
    )
}

fn create_handler_error(msg: String) -> HandlerError {
    io::Error::new(io::ErrorKind::Other, msg).into_handler_error()
}

impl<'a> Handler for CustomerHandler<'a> {

    fn handle(self, state: State) -> Box<HandlerFuture> {
            
        let result_f: HandlerFuture = self.client.get_customer("1".to_owned())
                .map(|customer| create_customer_response(&state, customer))
                .map(|res| (&state, res))
                .map_err(|msg| (&state, create_handler_error(msg)));

        Box::new(result_f)
    }
}