use redis_async::client::PairedConnection;

use futures::Future;
use futures::future::ok;

use model::Customer;

//TODO Move elsewhere
pub trait CustomerClient {
    fn get_customer(self, id: String) -> Future<Item = Customer, Error = String>;
}

pub struct RedisCustomerClient { client: PairedConnection }

impl RedisCustomerClient {
    pub fn new(c: PairedConnection) -> RedisCustomerClient {
        RedisCustomerClient { client: c }
    }
}

impl CustomerClient for RedisCustomerClient {
    fn get_customer(self, id: String) -> Future<Item = Customer, Error = String> {
        let hardcoded_customer = Customer { id: "1".to_owned(), first_name: "John".to_owned(), last_name: "Doe".to_owned() };
        ok::<Customer, String>(hardcoded_customer)
    }
}

