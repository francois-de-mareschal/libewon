use crate::m2web::{
    error,
    ewon::{ApiResponse, Ewon},
};
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
    /// Athenticate statefully or not.
    #[builder(default = "false")]
    stateful_auth: bool,
    /// Session id returned by the API in case of successful authentication.
    #[builder(default = "None", setter(skip))]
    t2m_session: Option<String>,
    /// HTTP client to connect to the API.
    #[builder(setter(strip_option, skip), default = "reqwest::Client::new()")]
    http_client: HttpClient,
}

impl<'a> Client<'a> {
    /// Open a stateful session.
    ///
    /// To remain compatible with potential legacy code which could use the stateful authentication, authenticate
    /// statefully against the M2Web API. The API will return a session id which will be the API key for subsequent
    /// calls of to the API.
    ///
    /// # Example
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn open_t2m_session_ok() -> Result<(), error::Error> {
    /// let client = ClientBuilder::default().stateful_auth(true).build()?;
    /// let _ = client.login().await?;
    ///
    /// // Do something useful, for example:
    /// let ewons = client.get_ewons(None).await?;
    /// # client.logout().await?;
    /// # }
    /// ```
    pub async fn login(&mut self) -> Result<&str, error::Error> {
        // Check if the user set the stateful auth.
        if !self.stateful_auth {
            return Err(error::Error {
                code: 500,
                kind: error::ErrorKind::StatelessAuthSet("stateful_auth was not set".to_string()),
            });
        }

        let api_response = self.request_api("login", None).await?;
        self.t2m_session = Some(api_response.t2msession.to_owned());

        Ok(&self.t2m_session.as_ref().unwrap())
    }

    /// Close a stateful session.
    ///
    /// Close the session once the querying of the API is complete. CLosing the session invalidates the
    /// session id and set it to `None`. All subsequent calls to the API will fail, unless `login()` is
    /// called again to get a new session id.
    ///
    /// To avoid the client to be called after a logout, the `logout()` methods consumes the `client`.
    ///
    /// # Example
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn close_t2m_session_ok() -> Result<(), error::Error> {
    /// let client = ClientBuilder::default().stateful_auth(true).build()?;
    /// let _ = client.login().await?;
    ///
    /// // Do something useful, for example:
    /// let ewons = client.get_ewons(None).await?;
    ///
    /// // All subsequent calls to the API will fail.
    /// client.logout().await?;
    /// # }
    /// ```
    pub async fn logout(mut self) -> Result<(), error::Error> {
        // Check if the user set the stateful auth.
        if !self.stateful_auth {
            return Err(error::Error {
                code: 500,
                kind: error::ErrorKind::StatelessAuthSet("stateful_auth was not set".to_string()),
            });
        }

        let _ = self.request_api("logout", None).await?;
        self.t2m_session = None;

        Ok(())
    }

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
    pub async fn get_ewons(&self, pool: Option<&str>) -> Result<Vec<Ewon>, error::Error> {
        let query_params = vec![("pool", pool.unwrap_or_default())];
        let api_response = self.request_api("getewons", Some(query_params)).await?;

        if api_response.ewons.is_empty() {
            Err(error::Error {
                code: 204,
                kind: error::ErrorKind::NoContent("No eWON were returned by API".to_string()),
            })
        } else {
            Ok(api_response.ewons)
        }
    }

    /// Return the eWON selected by its name.
    ///
    /// Get the eWON selected by its name and only this one. The name have to be the exact
    /// name of the eWON, like returned by `get_ewons()`.
    ///
    /// # Example
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn get_one_ewon_by_name() -> Result<Vec<Ewon>, error::Error> {
    /// // Get all eWONs belonging to the corporate account.
    /// let client = ClientBuilder::default().build()?;
    /// let ewon = client.get_ewon_by_name("ewon42").await?;
    ///
    /// // Do something useful, for example:
    /// println!("eWON name: {}", ewon.name);
    /// # }
    /// ```
    pub async fn get_ewon_by_name(&self, name: &str) -> Result<Ewon, error::Error> {
        let query_params = vec![("name", name)];
        let api_response = self.request_api("getewon", Some(query_params)).await?;

        Ok(api_response.ewon)
    }

    /// Return the eWON selected by its id.
    ///
    /// Get the eWON selected by its id and only this one. The id have to be the exact
    /// id of the eWON, like returned by `get_ewons()`.
    ///
    /// # Example
    /// ```rust
    /// # use libewon::m2web::{client::ClientBuilder, error, ewon::Ewon};
    /// # #[tokio::test]
    /// # async fn get_one_ewon_by_name() -> Result<Vec<Ewon>, error::Error> {
    /// // Get all eWONs belonging to the corporate account.
    /// let client = ClientBuilder::default().build()?;
    /// let ewon = client.get_ewon_by_id(42).await?;
    ///
    /// // Do something useful, for example:
    /// println!("eWON id: {}", ewon.id);
    /// # }
    /// ```
    pub async fn get_ewon_by_id(&self, id: u32) -> Result<Ewon, error::Error> {
        let id = id.to_string();
        let query_params = vec![("id", id.as_ref())];
        let api_response = self.request_api("getewon", Some(query_params)).await?;

        Ok(api_response.ewon)
    }

    /// Perform the request and check the HTTP error codes.
    async fn request_api(
        &self,
        url_path: &str,
        req_query_params: Option<Vec<(&str, &str)>>,
    ) -> Result<ApiResponse, error::Error> {
        // Check if the auth is stateful or not.
        let mut query_params = match self.stateful_auth {
            true => match url_path {
                // In case of stateful request, check if the user is performing a login.
                "login" => vec![
                    ("t2maccount", self.t2m_account),
                    ("t2musername", self.t2m_username),
                    ("t2mpassword", self.t2m_password),
                    ("t2mdeveloperid", self.t2m_developer_id),
                ],
                // If the user is querying anoter endpoint, return the session id.
                _ => {
                    if let Some(ref t2m_session) = self.t2m_session {
                        vec![
                            ("t2msession", t2m_session.as_ref()),
                            ("t2mdeveloperid", self.t2m_developer_id),
                        ]
                    } else {
                        // If the session id does not exist and the user is not performin a login, return an error.
                        return Err(error::Error {
                            code: 403,
                            kind: error::ErrorKind::InvalidCredentials(
                                "No session opened, please login before requesting the API"
                                    .to_string(),
                            ),
                        });
                    }
                }
            },
            // Return stateless authentication parameters.
            false => vec![
                ("t2maccount", self.t2m_account),
                ("t2musername", self.t2m_username),
                ("t2mpassword", self.t2m_password),
                ("t2mdeveloperid", self.t2m_developer_id),
            ],
        };

        if let Some(ref additional_query_params) = req_query_params {
            additional_query_params
                .iter()
                .for_each(|param| query_params.push(param.to_owned()));
        }

        let http_response = self
            .http_client
            .get(format!("{}/{}", self.t2m_url, url_path))
            .query(&query_params)
            .send()
            .await?;

        let http_status = http_response.status();
        let http_body = http_response.text().await?;
        let api_response = serde_json::from_str::<ApiResponse>(&http_body)?;

        match api_response.success {
            true => Ok(api_response),
            false => match http_status {
                reqwest::StatusCode::BAD_REQUEST => Err(error::Error {
                    code: http_status.as_u16(),
                    kind: error::ErrorKind::MissingParameter(format!("{}", api_response.message)),
                }),
                reqwest::StatusCode::FORBIDDEN => Err(error::Error {
                    code: http_status.as_u16(),
                    kind: error::ErrorKind::InvalidCredentials(format!("{}", api_response.message)),
                }),
                reqwest::StatusCode::GONE => Err(error::Error {
                    code: http_status.as_u16(),
                    kind: error::ErrorKind::EmptyResponse(format!("{}", api_response.message)),
                }),
                _ => Err(error::Error {
                    code: 500,
                    kind: error::ErrorKind::UnknownError("Unkown error occurred".to_string()),
                }),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::m2web::{client, error, ewon};
    use serde_json::json;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn get_ewons_empty_ok() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        let json_response = json!({
          "ewons": [],
          "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(query_param("pool", ""))
            .and(path("/t2mapi/getewons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response))
            .expect(1)
            .mount(&server)
            .await;

        let ewons = match client.get_ewons(None).await {
            Ok(_) => panic!("client.get_ewons(None) should had returned an HTTP 204"),
            Err(err) => err,
        };

        assert_eq!(
            error::Error {
                code: 204,
                kind: error::ErrorKind::NoContent("No eWON were returned by API".to_string())
            },
            ewons
        );

        Ok(())
    }

    #[tokio::test]
    async fn get_ewons_filled_ok() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        let json_response = json!({
          "ewons": [
            {
              "id": 1206698,
              "name": "bea-test",
              "encodedName": "bea-test",
              "status": "offline",
              "description": "",
              "customAttributes": [
                "bea",
                "",
                ""
              ],
              "m2webServer": "eu2.m2web.talk2m.com",
              "lanDevices": [],
              "ewonServices": []
            },
            {
              "id": 639491,
              "name": "eWON  FLEXOCOLOR SM2845",
              "encodedName": "eWON++FLEXOCOLOR+SM2845",
              "status": "online",
              "description": "SM2845 SIRIUS DEBOBINEUR1000",
              "customAttributes": [
                "FRANCE",
                "",
                ""
              ],
              "m2webServer": "eu2.m2web.talk2m.com",
              "lanDevices": [],
              "ewonServices": []
            }
          ],
          "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(query_param("pool", ""))
            .and(path("/t2mapi/getewons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response))
            .expect(1)
            .mount(&server)
            .await;

        let ewons = client.get_ewons(None).await?;

        assert_eq!(
            &vec![
                ewon::Ewon {
                    id: 1206698,
                    name: "bea-test".to_string(),
                    encoded_name: "bea-test".to_string(),
                    status: "offline".to_string(),
                    description: "".to_string(),
                    custom_attributes: ["bea".to_string(), "".to_string(), "".to_string(),],
                    m2web_server: "eu2.m2web.talk2m.com".to_string(),
                    lan_devices: vec![],
                    ewon_services: vec![],
                },
                ewon::Ewon {
                    id: 639491,
                    name: "eWON  FLEXOCOLOR SM2845".to_string(),
                    encoded_name: "eWON++FLEXOCOLOR+SM2845".to_string(),
                    status: "online".to_string(),
                    description: "SM2845 SIRIUS DEBOBINEUR1000".to_string(),
                    custom_attributes: ["FRANCE".to_string(), "".to_string(), "".to_string(),],
                    m2web_server: "eu2.m2web.talk2m.com".to_string(),
                    lan_devices: vec![],
                    ewon_services: vec![],
                }
            ],
            &ewons
        );

        Ok(())
    }

    #[tokio::test]
    async fn get_ewons_filled_missing_fields_ko() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .build()
            .unwrap();

        let json_response = json!({
          "ewons": [
            {
              "id": 1206698,
              "name": "bea-test",
              "encodedName": "bea-test",
              "customAttributes": [
                "bea",
                "",
                ""
              ],
              "m2webServer": "eu2.m2web.talk2m.com",
              "lanDevices": [],
              "ewonServices": []
            },
            {
              "id": 639491,
              "name": "eWON  FLEXOCOLOR SM2845",
              "encodedName": "eWON++FLEXOCOLOR+SM2845",
              "status": "online",
              "description": "SM2845 SIRIUS DEBOBINEUR1000",
              "lanDevices": [],
              "ewonServices": []
            }
          ],
          "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(query_param("pool", ""))
            .and(path("/t2mapi/getewons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response))
            .expect(1)
            .mount(&server)
            .await;

        let ewons = match client.get_ewons(None).await {
            Ok(_) => panic!("client.get_ewons(None) should had returned error::ResponseParsing"),
            Err(err) => err,
        };

        assert_eq!(
            error::Error {
                code: 500,
                kind: error::ErrorKind::ResponseParsing(String::from(
                    "JSON response data format does not match the expected one: missing field `status` at line 1 column 171"
                ))
            },
            ewons
        );

        Ok(())
    }

    #[tokio::test]
    async fn config_stateful_login_ko() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let mut client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .stateful_auth(true)
            .build()
            .unwrap();

        let json_response = json!({
          "code": 403,
          "message": "Invalid credentials",
          "success": false
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account1"))
            .and(query_param("t2musername", "username1"))
            .and(query_param("t2mpassword", "password1"))
            .and(query_param(
                "t2mdeveloperid",
                "731e38ec-981f-4f31-9cb5-e87f0d571816",
            ))
            .and(path("/t2mapi/login"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&json_response))
            .expect(1)
            .mount(&server)
            .await;

        let session_id = match client.login().await {
            Ok(_) => {
                panic!("client.login().await should had returned an error::InvalidCredentials")
            }
            Err(err) => err,
        };

        assert_eq!(
            session_id,
            error::Error {
                code: 403,
                kind: error::ErrorKind::InvalidCredentials("Invalid credentials".to_string()),
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn config_stateful_login_ok() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let mut client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .stateful_auth(true)
            .build()
            .unwrap();

        let json_response = json!({
          "t2msession": "e44be62aaa9381707b5ab328c18d4a43",
          "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response))
            .expect(1)
            .mount(&server)
            .await;

        let session_id = client.login().await?;

        assert_eq!(session_id, "e44be62aaa9381707b5ab328c18d4a43");

        Ok(())
    }

    #[tokio::test]
    async fn config_stateless_login_ko() -> Result<(), error::Error> {
        let mut client = client::ClientBuilder::default().build().unwrap();

        let session_id = match client.login().await {
            Ok(_) => {
                panic!("client.login().await should had returned an error::StatelessAuthSet")
            }
            Err(err) => err,
        };

        assert_eq!(
            session_id,
            error::Error {
                code: 500,
                kind: error::ErrorKind::StatelessAuthSet("stateful_auth was not set".to_string()),
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn config_stateful_logout_ok() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let mut client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .stateful_auth(true)
            .build()
            .unwrap();

        let json_response_login = json!({
          "t2msession": "e44be62aaa9381707b5ab328c18d4a43",
          "success": true
        });

        let json_response_logout = json!({
            "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response_login))
            .expect(1)
            .named("login")
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(query_param(
                "t2msession",
                "e44be62aaa9381707b5ab328c18d4a43",
            ))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response_logout))
            .expect(1)
            .named("logout")
            .mount(&server)
            .await;

        let _ = client.login().await?;
        client.logout().await?;

        Ok(())
    }

    #[tokio::test]
    async fn config_stateful_logout_ko_wrong_session_id() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let mut client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .stateful_auth(true)
            .build()
            .unwrap();

        let json_response_login = json!({
          "t2msession": "e44be62aaa9381707b5ab328c18d4a43",
          "success": true
        });

        let json_response_logout = json!({
          "message": "Session ID [0c94ef0b19b0c3b596115d4e5e1d2d02] is invalid",
          "code": 403,
          "success": false
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response_login))
            .expect(1)
            .named("login")
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(query_param(
                "t2msession",
                "0c94ef0b19b0c3b596115d4e5e1d2d02",
            ))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/logout"))
            .respond_with(ResponseTemplate::new(403).set_body_json(&json_response_logout))
            .expect(1)
            .named("logout")
            .mount(&server)
            .await;

        let _ = client.login().await?;
        client.t2m_session = Some("0c94ef0b19b0c3b596115d4e5e1d2d02".to_string());
        let logout = client.logout().await;

        assert_eq!(
            logout,
            Err(error::Error {
                code: 403,
                kind: error::ErrorKind::InvalidCredentials(
                    "Session ID [0c94ef0b19b0c3b596115d4e5e1d2d02] is invalid".to_string()
                )
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn config_stateful_logout_ko_no_session_id() -> Result<(), error::Error> {
        let server = MockServer::start().await;
        let server_uri = format!("{}/t2mapi", &server.uri());
        let mut client = client::ClientBuilder::default()
            .t2m_url(&server_uri)
            .t2m_account("account2")
            .t2m_username("username2")
            .t2m_password("password2")
            .t2m_developer_id("795f1844-2f5e-4d8b-9922-25c45d3e1c47")
            .stateful_auth(true)
            .build()
            .unwrap();

        let json_response_login = json!({
          "t2msession": "e44be62aaa9381707b5ab328c18d4a43",
          "success": true
        });

        let json_response_logout = json!({
          "success": true
        });

        Mock::given(method("GET"))
            .and(query_param("t2maccount", "account2"))
            .and(query_param("t2musername", "username2"))
            .and(query_param("t2mpassword", "password2"))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/login"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response_login))
            .expect(1)
            .named("login")
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(query_param(
                "t2msession",
                "e44be62aaa9381707b5ab328c18d4a43",
            ))
            .and(query_param(
                "t2mdeveloperid",
                "795f1844-2f5e-4d8b-9922-25c45d3e1c47",
            ))
            .and(path("/t2mapi/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json_response_logout))
            .expect(0)
            .named("logout")
            .mount(&server)
            .await;

        let _ = client.login().await?;
        client.t2m_session = None;
        let logout = client.logout().await;

        assert_eq!(
            logout,
            Err(error::Error {
                code: 403,
                kind: error::ErrorKind::InvalidCredentials(
                    "No session opened, please login before requesting the API".to_string(),
                ),
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn config_stateless_logout_ko() -> Result<(), error::Error> {
        let client = client::ClientBuilder::default().build().unwrap();

        let session_id = match client.logout().await {
            Ok(_) => {
                panic!("client.login().await should had returned an error::StatelessAuthSet")
            }
            Err(err) => err,
        };

        assert_eq!(
            session_id,
            error::Error {
                code: 500,
                kind: error::ErrorKind::StatelessAuthSet("stateful_auth was not set".to_string()),
            }
        );

        Ok(())
    }
}
