use derive_builder::Builder;

/// M2Web API client.
///
/// Connect to the M2Web API. Hold connection parameters, API endpoints, and connection method.
#[derive(Builder)]
pub struct Client<'a> {
    /// The API base url.
    #[builder(setter(strip_option), default = "\"https://m2web.talk2m.com/t2mapi\"")]
    t2m_url: &'a str,
}

#[cfg(test)]
mod test {
    use crate::m2web::client;

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
}
