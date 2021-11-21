use derive_builder::Builder;
use reqwest::Client as HttpClient;

/// M2Web API client.
///
/// Connect to the M2Web API. Hold connection parameters, API endpoints, and connection method.
#[derive(Builder)]
pub struct Client<'a> {
    /// The API base url.
    #[builder(setter(strip_option), default = "\"https://m2web.talk2m.com/t2mapi\"")]
    t2m_url: &'a str,
    /// The Talk2M corporate account.
    t2m_account: &'a str,
    /// The Talk2M user attached to the corporate account.
    t2m_username: &'a str,
    /// The password attached to the username.
    t2m_password: &'a str,
    /// The Talk2M API key used to check the user is authorized to use the API.
    t2m_developer_id: &'a str,
    /// HTTP client to connect to the API.
    #[builder(setter(strip_option), default = "reqwest::Client::new()")]
    http_client: HttpClient,
}

#[cfg(test)]
mod test {
    use crate::m2web::client;
    use wiremock::{matchers::method, Mock, MockServer, ResponseTemplate};

    #[test]
    fn build_client_default_t2m_url() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default().build()?;
        assert_eq!(client.t2m_url, "https://m2web.talk2m.com/t2mapi");

        Ok(())
    }

    #[test]
    fn build_client_custom_t2m_url() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default()
            .t2m_url("https://data.talk2m.com")
            .build()?;
        assert_eq!(client.t2m_url, "https://data.talk2m.com");

        Ok(())
    }

    #[tokio::test]
    async fn client_connect_to_api_base_url() -> Result<(), reqwest::Error> {
        let server = MockServer::start().await;
        let server_uri = &server.uri();

        let client = client::ClientBuilder::default()
            .t2m_url(server_uri)
            .build()
            .unwrap();
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let status = client.http_client.get(server_uri).send().await?.status();

        assert_eq!(status, reqwest::StatusCode::OK);

        Ok(())
    }
}
