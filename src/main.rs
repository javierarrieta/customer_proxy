
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use gotham::http::response::create_response;
use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::*;
use hyper::{Response, StatusCode};

fn main() {
    let customer = Customer { id: "1".to_owned(), first_name: "John".to_owned(), last_name: "Doe".to_owned() };

    let addr = "0.0.0.0:9000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router(&customer))
}


#[derive(Serialize)]
struct Customer {
    id: String,
    first_name: String,
    last_name: String,
}

fn router(customer: &Customer) -> Router {
    build_simple_router(|route| {
        route.get("/person").to(customer_handler(customer));
    })
}

fn customer_handler(customer: &Customer) -> impl Fn(State) -> (State, Response) {
    move |state|
    (
        state,
        create_response(
            &state,
            StatusCode::Ok,
            Some((
                serde_json::to_vec(&product).expect("serialized product"),
                mime::APPLICATION_JSON,
            )),
        )
    )
}