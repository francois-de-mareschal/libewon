use crate::m2web::{error, ewon::Ewon};
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
    #[builder(default = "\"account1\"")]
    t2m_account: &'a str,
    /// The Talk2M user attached to the corporate account.
    #[builder(default = "\"username1\"")]
    t2m_username: &'a str,
    /// The password attached to the username.
    #[builder(default = "\"password1\"")]
    t2m_password: &'a str,
    /// The Talk2M API key used to check the user is authorized to use the API.
    #[builder(default = "\"731e38ec-981f-4f31-9cb5-e87f0d571816\"")]
    t2m_developer_id: &'a str,
    /// HTTP client to connect to the API.
    #[builder(setter(strip_option), default = "reqwest::Client::new()")]
    http_client: HttpClient,
}

impl<'a> Client<'a> {
    /// Return the list of all eWONs registered for the corporate account.
    ///
    /// The M2Web API allows to get the list of all eWONs associated to the corporate account used
    /// to connect to. By default, all eWONs are returned, unless an optional pool name if specified
    /// to the function; only the eWONs belonging to this pool will be returned.
    ///
    /// # Example
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn get_all_ewons_from_all_pools() -> Result<Vec<Ewon>, error::Error> {
    /// // Get all eWONs belonging to the corporate account.
    /// let client = ClientBuilder::default().build()?;
    /// let all_ewons = client.get_ewons(None).await?;
    /// # }
    /// ```
    ///
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn get_all_ewons_from_specific_pool() -> Result<Vec<Ewon>, error::Error> {
    /// // Get all eWONs belonging to the corporate account and the "emea" pool.
    /// let client = ClientBuilder::default().build()?;
    /// let all_ewons = client.get_ewons(Some("emea")).await?;
    /// # }
    /// ```
    pub async fn get_ewons(&self, pool: Option<&str>) -> Result<Vec<Ewon<'_>>, error::Error> {
        Err(error::Error::new(
            403,
            error::ErrorKind::InvalidCredentials(String::from("Invalid credentials")),
        ))
    }

    /// Build the authentication parameters to be passed in the url.
    fn build_url_stateless_auth_params(&self) -> String {
        let mut parameters = String::new();

        parameters.push_str(&format!("t2maccount={}", self.t2m_account));
        parameters.push('&');
        parameters.push_str(&format!("t2musername={}", self.t2m_username));
        parameters.push('&');
        parameters.push_str(&format!("t2mpassword={}", self.t2m_password));
        parameters.push('&');
        parameters.push_str(&format!("t2mdeveloperid={}", self.t2m_developer_id));

        parameters
    }

    /// Build the URL with base, parameters and endpoint.
    fn build_url(&self) -> String {
        let mut request_url = String::new();

        request_url.push_str(self.t2m_url);
        // TODO: build endpoint.
        request_url.push('?');
        request_url.push_str(&self.build_url_stateless_auth_params());

        request_url
    }
}

#[cfg(test)]
mod test {
    use crate::m2web::{client, error};
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

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

    #[test]
    fn build_client_default_account() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default().build()?;
        assert_eq!(client.t2m_account, "account1");

        Ok(())
    }

    #[test]
    fn build_client_custom_account() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default()
            .t2m_account("account2")
            .build()?;
        assert_eq!(client.t2m_account, "account2");

        Ok(())
    }

    #[test]
    fn build_client_default_username() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default().build()?;
        assert_eq!(client.t2m_username, "username1");

        Ok(())
    }

    #[test]
    fn build_client_custom_username() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default()
            .t2m_username("username2")
            .build()?;
        assert_eq!(client.t2m_username, "username2");

        Ok(())
    }

    #[test]
    fn build_client_default_password() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default().build()?;
        assert_eq!(client.t2m_password, "password1");

        Ok(())
    }

    #[test]
    fn build_client_custom_password() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default()
            .t2m_password("password2")
            .build()?;
        assert_eq!(client.t2m_password, "password2");

        Ok(())
    }

    #[test]
    fn build_client_default_developer_id() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default().build()?;
        assert_eq!(
            client.t2m_developer_id,
            "731e38ec-981f-4f31-9cb5-e87f0d571816"
        );

        Ok(())
    }

    #[test]
    fn build_client_custom_developer_id() -> Result<(), client::ClientBuilderError> {
        let client = client::ClientBuilder::default()
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()?;
        assert_eq!(
            client.t2m_developer_id,
            "795f1844-2f5e-4d8b-9922-25c45d3e1c47"
        );

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

    #[test]
    fn build_stateless_auth_params_url_default() {
        let generated_auth_params = "t2maccount=account1&t2musername=username1&t2mpassword=password1&t2mdeveloperid=731e38ec-981f-4f31-9cb5-e87f0d571816";
        let client = client::ClientBuilder::default().build().unwrap();

        let auth_params = client.build_url_stateless_auth_params();

        assert_eq!(generated_auth_params, &auth_params);
    }

    #[test]
    fn build_stateless_auth_params_url_custom() {
        let generated_auth_params = "t2maccount=account2&t2musername=username2&t2mpassword=password2&t2mdeveloperid=795f1844-2f5e-4d8b-9922-25c45d3e1c47";
        let client = client::ClientBuilder::default()
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        let auth_params = client.build_url_stateless_auth_params();

        assert_eq!(generated_auth_params, &auth_params);
    }

    #[test]
    fn build_url_default_params_no_endpoint() {
        let generated_url = "https://m2web.talk2m.com/t2mapi?t2maccount=account1&t2musername=username1&t2mpassword=password1&t2mdeveloperid=731e38ec-981f-4f31-9cb5-e87f0d571816";
        let client = client::ClientBuilder::default().build().unwrap();

        let url = client.build_url();

        assert_eq!(generated_url, url);
    }

    #[test]
    fn build_url_custom_params_no_endpoint() {
        let generated_url = "https://m2web.talk2m.com/t2mapi?t2maccount=account2&t2musername=username2&t2mpassword=password2&t2mdeveloperid=795f1844-2f5e-4d8b-9922-25c45d3e1c47";
        let client = client::ClientBuilder::default()
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        let url = client.build_url();

        assert_eq!(generated_url, url);
    }

    #[tokio::test]
    async fn stateless_auth_default_params_ko() -> Result<(), reqwest::Error> {
        let server = MockServer::start().await;
        let server_uri = &server.uri();

        let client = client::ClientBuilder::default()
            .t2m_url(server_uri)
            .build()
            .unwrap();
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;
        let url = client.build_url();
        let status = client.http_client.get(url).send().await?.status();

        assert_eq!(status, reqwest::StatusCode::FORBIDDEN);

        Ok(())
    }

    #[tokio::test]
    async fn stateless_auth_custom_params_ok() -> Result<(), reqwest::Error> {
        let server = MockServer::start().await;
        let server_uri = &server.uri();

        let client = client::ClientBuilder::default()
            .t2m_url(server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();
        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let url = client.build_url();
        let status = client.http_client.get(url).send().await?.status();

        assert_eq!(status, reqwest::StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn get_ewons_empty_ok() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = &server.uri();
        let client = client::ClientBuilder::default()
            .t2m_url(server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/getewons"))
            .respond_with(ResponseTemplate::new(200).set_body_json("{}"))
            .mount(&server)
            .await;

        let ewons = client.get_ewons(None).await?;
        assert_eq!(Vec::<client::Ewon>::new(), ewons);

        Ok(())
    }
}
