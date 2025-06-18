use url::Host;

use crate::Url;

impl Url {
    pub fn match_host(&self, host: &Host<&str>) -> bool {
        url::Url::parse(&self.ressource)
            .expect("Malformed url found")
            .host()
            .is_some_and(|url_host| {
                // test
                host == &url_host
            })
    }
}
