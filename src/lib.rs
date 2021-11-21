//! Client for the M2Web REST API.
//!
//! The M2Web REST API provides access and control to the eWONs which belong to the user. The client allows
//! to connect to the M2Web API, to get the details of all the eWONs or one in particular, to get properties
//! from the eWON or to send it commands.
//!
//! The developer documentation for the M2Web REST API could be found
//! [here](https://developer.ewon.biz/content/m2web-api-0).
//!
//! # Example
//! ```rust
//! # use libewon::m2web::client::ClientBuilder;
//! let _client = ClientBuilder::default()
//!     .t2m_url("https://m2web.talk2m.com/t2mapi")
//!     .t2m_account("account1")
//!     .t2m_username("username1")
//!     .t2m_password("password1")
//!     .t2m_developer_id("731e38ec-981f-4f31-9cb5-e87f0d571816")
//!     .build()
//!     .unwrap();
//! ```

pub mod m2web;
