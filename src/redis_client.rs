use redis_async::client::PairedConnection;

use futures::Future;

use model::Customer;

//TODO Move elsewhere
pub trait CustomerClient {
    fn get_customer(&self, id: &str) -> Future<Item = Customer, Error = String>;
}

pub struct RedisCustomerClient { client: PairedConnection }

impl RedisCustomerClient {
    fn new(c: PairedConnection) -> RedisCustomerClient {
        RedisCustomerClient { client: c }
    }
}

impl CustomerClient for RedisCustomerClient {
    fn get_customer(&self, id: &str) -> Future<Item = Customer, Error = String> {
        futures::future::ok(Customer { id: "1", first_name: "John", last_name: "Doe" } )
    }
}

