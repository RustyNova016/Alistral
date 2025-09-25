use crate::DBClient;


pub fn test_mb_client() -> DBClient {
    DBClient::get_default_named("test").unwrap()
}
