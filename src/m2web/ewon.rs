use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Response from the M2Web API.
///
/// Handle the encapsulated response from the API to extract the data.
#[derive(Debug, Serialize, Deserialize)]
pub(in crate::m2web) struct ApiResponse {
    /// Indicates if the request suceeded or not.
    pub(in crate::m2web) success: bool,
    /// A specific eWON have been requested from the API.
    #[serde(default)]
    pub(in crate::m2web) ewon: Ewon,
    /// All eWON or eWON from a pool have been requested from the API.
    #[serde(default)]
    pub(in crate::m2web) ewons: Vec<Ewon>,
    /// Session id returned by the API in case of stateful auth.
    #[serde(default)]
    pub(in crate::m2web) t2msession: String,
    /// Message to explain which error just happened.
    #[serde(default)]
    pub(in crate::m2web) message: String,
}

/// eWON parameters.
///
/// Each eWON is registered and identified by these parameters.
#[derive(Builder, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ewon {
    /// The UID of the eWON to the M2Web API.
    pub id: u32,
    /// The unique name of the eWON.
    pub name: String,
    /// The url-encoded name of the eWON.
    pub encoded_name: String,
    /// The status of the eWON, either connected or disconnected.
    pub status: String,
    /// The user description of the eWON.
    pub description: String,
    /// The three user-customized attributes of the eWON.
    pub custom_attributes: [String; 3],
    /// The M2Web VPN server on which the eWON is connected to.
    pub m2web_server: String,
    /// The LAN devices connected to the eWON.
    pub lan_devices: Vec<String>,
    /// The active eWON services.
    pub ewon_services: Vec<String>,
}
