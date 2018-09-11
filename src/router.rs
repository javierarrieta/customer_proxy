
use gotham::router::Router;
use gotham::router::builder::*;
use model::Customer;
use handlers::customer_handler;

pub fn customer_router(customer: &Customer) -> Router {
    build_simple_router(|route| {
        route.get("/person").to_new_handler(|s| customer_handler(customer, s));
    })
}