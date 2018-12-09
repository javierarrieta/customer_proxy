use gotham::router::Router;
use gotham::router::builder::*;
use handlers::CustomerHandler;

pub fn customer_router(customer_handler: CustomerHandler) -> Router {
    build_simple_router(|route| {
        route.get("/customer").to(customer_handler);
    })
}