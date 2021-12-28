use libewon::m2web::{client, error, ewon};
use serde_json::json;
use wiremock::{
    matchers::{method, path, query_param},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn get_ewon_by_id_empty_ok() -> Result<(), error::Error> {
    let server = wiremock::MockServer::start().await;
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
      "message": "Device [] does not exist",
      "code": 410,
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
        .and(query_param("id", "42"))
        .and(path("/t2mapi/getewon"))
        .respond_with(ResponseTemplate::new(410).set_body_json(&json_response))
        .expect(1)
        .mount(&server)
        .await;

    let ewon = match client.get_ewon_by_id(42).await {
        Ok(_) => panic!("get_ewon_by_id should have returned an error::Error 410"),
        Err(err) => err,
    };

    assert_eq!(format!("{}", ewon), "HTTP 410: Device [] does not exist",);

    Ok(())
}

#[tokio::test]
async fn get_ewon_by_id_filled_ok() -> Result<(), error::Error> {
    let server = wiremock::MockServer::start().await;
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
      "ewon": {
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
        .and(query_param("id", "1206698"))
        .and(path("/t2mapi/getewon"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&json_response))
        .expect(1)
        .mount(&server)
        .await;

    let ewon = client.get_ewon_by_id(1206698).await?;

    assert_eq!(
        ewon,
        ewon::Ewon {
            id: 1206698,
            name: "bea-test".to_string(),
            encoded_name: "bea-test".to_string(),
            status: "offline".to_string(),
            description: "".to_string(),
            custom_attributes: ["bea".to_string(), "".to_string(), "".to_string()],
            m2web_server: "eu2.m2web.talk2m.com".to_string(),
            lan_devices: vec![],
            ewon_services: vec![],
        }
    );

    Ok(())
}
