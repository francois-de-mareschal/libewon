use derive_builder::Builder;
/// eWON parameters.
///
/// Each eWON is registered and identified by these parameters.
#[derive(Builder, Debug, PartialEq)]
pub struct Ewon<'a> {
    /// The UID of the eWON to the M2Web API.
    id: u32,
    /// The unique name of the eWON.
    name: &'a str,
    /// The url-encoded name of the eWON.
    encoded_name: &'a str,
    /// The status of the eWON, either connected or disconnected.
    status: EwonStatus,
    /// The user description of the eWON.
    description: &'a str,
    /// The three user-customized attributes of the eWON.
    custom_attributes: [&'a str; 3],
    /// The M2Web VPN server on which the eWON is connected to.
    m2web_server: &'a str,
    /// The LAN devices connected to the eWON.
    lan_devices: Vec<&'a str>,
    /// The active eWON services.
    ewon_services: Vec<&'a str>,
}

/// The current status of the eWON, either connected or disconnected.
#[derive(Clone, Debug, PartialEq)]
pub enum EwonStatus {
    /// The eWON is currently online.
    Connected,
    /// The eWON is currently offline.
    Disconnected,
}
