use libewon::m2web::{client, error};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

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
    client.logout().await.unwrap();

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
        format!("{}", session_id),
        "Client set to authenticate statelessly: stateful_auth was not set"
    );

    Ok(())
}
