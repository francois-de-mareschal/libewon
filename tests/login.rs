use libewon::m2web::{client, error};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

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

    assert_eq!(format!("{}", session_id), "HTTP 403: Invalid credentials");

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
        format!("{}", session_id),
        "Client set to authenticate statelessly: stateful_auth was not set"
    );

    Ok(())
}
