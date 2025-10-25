use std::sync::Arc;

pub fn make_client() -> Arc<reqwest::Client> {
    Arc::new(reqwest::Client::new())
}
