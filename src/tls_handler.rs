use isahc::HttpClient;
use isahc::config::{SslOption, Configurable};

pub fn create_client() -> String {
    //SINK
    let _client = HttpClient::builder().ssl_options(SslOption::DANGER_ACCEPT_INVALID_CERTS)
        .build();

    "client created".to_string()
}
