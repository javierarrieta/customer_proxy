extern crate gotham;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate mime;

use model::Customer;

use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{StatusCode, Response};

pub fn customer_handler(customer: &Customer, state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((
            serde_json::to_vec(customer).expect("serialized product"),
            mime::APPLICATION_JSON,
        )),
    );
    (state, res)
}