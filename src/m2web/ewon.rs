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
}

/// eWON parameters.
///
/// Each eWON is registered and identified by these parameters.
#[derive(Builder, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ewon {
    /// The UID of the eWON to the M2Web API.
    pub(in crate::m2web) id: u32,
    /// The unique name of the eWON.
    pub(in crate::m2web) name: String,
    /// The url-encoded name of the eWON.
    pub(in crate::m2web) encoded_name: String,
    /// The status of the eWON, either connected or disconnected.
    pub(in crate::m2web) status: String,
    /// The user description of the eWON.
    pub(in crate::m2web) description: String,
    /// The three user-customized attributes of the eWON.
    pub(in crate::m2web) custom_attributes: [String; 3],
    /// The M2Web VPN server on which the eWON is connected to.
    pub(in crate::m2web) m2web_server: String,
    /// The LAN devices connected to the eWON.
    pub(in crate::m2web) lan_devices: Vec<String>,
    /// The active eWON services.
    pub(in crate::m2web) ewon_services: Vec<String>,
}

/// The current status of the eWON, either connected or disconnected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EwonStatus {
    /// The eWON is currently online.
    Offline,
    /// The eWON is currently offline.
    Online,
}
