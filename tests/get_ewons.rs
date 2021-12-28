use libewon::m2web::{client, error, ewon};
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
        format!("{}", ewons),
        "HTTP 204: No eWON were returned by API"
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
        format!("{}", ewons),
        "Unable to parse JSON response: JSON response data format does not match the expected one: missing field `status` at line 1 column 171"
    );

    Ok(())
}
