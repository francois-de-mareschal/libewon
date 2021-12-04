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
    pub(in crate::m2web) ewon: Ewon,
    /// All eWON or eWON from a pool have been requested from the API.
    pub(in crate::m2web) ewons: Vec<Ewon>,
}

/// eWON parameters.
///
/// Each eWON is registered and identified by these parameters.
#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ewon {
    /// The UID of the eWON to the M2Web API.
    id: u32,
    /// The unique name of the eWON.
    name: String,
    /// The url-encoded name of the eWON.
    encoded_name: String,
    /// The status of the eWON, either connected or disconnected.
    status: EwonStatus,
    /// The user description of the eWON.
    description: String,
    /// The three user-customized attributes of the eWON.
    custom_attributes: [String; 3],
    /// The M2Web VPN server on which the eWON is connected to.
    m2web_server: String,
    /// The LAN devices connected to the eWON.
    lan_devices: Vec<String>,
    /// The active eWON services.
    ewon_services: Vec<String>,
}

/// The current status of the eWON, either connected or disconnected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EwonStatus {
    /// The eWON is currently online.
    Connected,
    /// The eWON is currently offline.
    Disconnected,
}
