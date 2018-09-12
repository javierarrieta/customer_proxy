#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate gotham;
extern crate hyper;
extern crate futures;

mod model;
mod handlers;
mod router;

use model::Customer;

fn main() {
    let customer: Customer = Customer { id: "1".to_owned(), first_name: "John".to_owned(), last_name: "Doe".to_owned() };

    let addr = "0.0.0.0:9000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router::customer_router(handlers::CustomerHandler { customer: customer }))
}