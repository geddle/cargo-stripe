// ======================================
// This file was automatically generated.
// ======================================

use crate::stripe::client::{Client, Response};
use crate::stripe::ids::{TerminalConfigurationId};
use crate::stripe::params::{Expand, Expandable, List, Object, Paginable};
use crate::stripe::resources::{File};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TerminalConfigurationConfiguration".
///
/// For more details see <https://stripe.com/docs/api/terminal/configuration/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfiguration {
    /// Unique identifier for the object.
    pub id: TerminalConfigurationId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Whether this Configuration is the default for your account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// String indicating the name of the Configuration object, set by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<TerminalConfigurationConfigurationResourceOfflineConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_window: Option<TerminalConfigurationConfigurationResourceRebootWindow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_s700: Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<TerminalConfigurationConfigurationResourceTipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<TerminalConfigurationConfigurationResourceWifiConfig>,
}

impl TerminalConfiguration {

    /// Returns a list of `Configuration` objects.
pub fn list(client: &Client, params: &ListTerminalConfigurations<'_>) -> Response<List<TerminalConfiguration>> {
   client.get_query("/terminal/configurations", params)
}


    /// Creates a new `Configuration` object.
    pub fn create(client: &Client, params: CreateTerminalConfiguration<'_>) -> Response<TerminalConfiguration> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/terminal/configurations", &params)
    }
}

impl Object for TerminalConfiguration {
    type Id = TerminalConfigurationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "terminal.configuration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {

    /// A File ID representing an image to display on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceOfflineConfig {

    /// Determines whether to allow transactions to be collected while reader is offline.
    ///
    /// Defaults to false.
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceRebootWindow {

    /// Integer between 0 to 23 that represents the end hour of the reboot time window.
    ///
    /// The value must be different than the start_hour.
    pub end_hour: i64,

    /// Integer between 0 to 23 that represents the start hour of the reboot time window.
    pub start_hour: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpy: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pln: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceWifiConfig {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_peap: Option<TerminalConfigurationConfigurationResourceEnterprisePeapWifi>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_tls: Option<TerminalConfigurationConfigurationResourceEnterpriseTlsWifi>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_psk: Option<TerminalConfigurationConfigurationResourcePersonalPskWifi>,

    /// Security type of the WiFi network.
    ///
    /// The hash with the corresponding name contains the credentials for this security type.
    #[serde(rename = "type")]
    pub type_: TerminalConfigurationConfigurationResourceWifiConfigType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceEnterprisePeapWifi {

    /// A File ID representing a PEM file containing the server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,

    /// Password for connecting to the WiFi network.
    pub password: String,

    /// Name of the WiFi network.
    pub ssid: String,

    /// Username for connecting to the WiFi network.
    pub username: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourceEnterpriseTlsWifi {

    /// A File ID representing a PEM file containing the server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,

    /// A File ID representing a PEM file containing the client certificate.
    pub client_certificate_file: String,

    /// A File ID representing a PEM file containing the client RSA private key.
    pub private_key_file: String,

    /// Password for the private key file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_file_password: Option<String>,

    /// Name of the WiFi network.
    pub ssid: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TerminalConfigurationConfigurationResourcePersonalPskWifi {

    /// Password for connecting to the WiFi network.
    pub password: String,

    /// Name of the WiFi network.
    pub ssid: String,
}

/// The parameters for `TerminalConfiguration::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateTerminalConfiguration<'a> {

    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<CreateTerminalConfigurationBbposWiseposE>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Name of the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// Configurations for collecting transactions offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateTerminalConfigurationOffline>,

    /// Reboot time settings for readers that support customized reboot time configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_window: Option<CreateTerminalConfigurationRebootWindow>,

    /// An object containing device type specific settings for Stripe S700 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_s700: Option<CreateTerminalConfigurationStripeS700>,

    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<CreateTerminalConfigurationTipping>,

    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<CreateTerminalConfigurationVerifoneP400>,

    /// Configurations for connecting to a WiFi network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<CreateTerminalConfigurationWifi>,
}

impl<'a> CreateTerminalConfiguration<'a> {
    pub fn new() -> Self {
        CreateTerminalConfiguration {
            bbpos_wisepos_e: Default::default(),
            expand: Default::default(),
            name: Default::default(),
            offline: Default::default(),
            reboot_window: Default::default(),
            stripe_s700: Default::default(),
            tipping: Default::default(),
            verifone_p400: Default::default(),
            wifi: Default::default(),
        }
    }
}

/// The parameters for `TerminalConfiguration::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListTerminalConfigurations<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<TerminalConfigurationId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// if present, only return the account default or non-default configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<TerminalConfigurationId>,
}

impl<'a> ListTerminalConfigurations<'a> {
    pub fn new() -> Self {
        ListTerminalConfigurations {
            ending_before: Default::default(),
            expand: Default::default(),
            is_account_default: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListTerminalConfigurations<'_> {
    type O = TerminalConfiguration;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationBbposWiseposE {

    /// A File ID representing an image to display on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationOffline {

    /// Determines whether to allow transactions to be collected while reader is offline.
    ///
    /// Defaults to false.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationRebootWindow {

    /// Integer between 0 to 23 that represents the end hour of the reboot time window.
    ///
    /// The value must be different than the start_hour.
    pub end_hour: i64,

    /// Integer between 0 to 23 that represents the start hour of the reboot time window.
    pub start_hour: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationStripeS700 {

    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTipping {

    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<CreateTerminalConfigurationTippingAud>,

    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<CreateTerminalConfigurationTippingCad>,

    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<CreateTerminalConfigurationTippingChf>,

    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<CreateTerminalConfigurationTippingCzk>,

    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<CreateTerminalConfigurationTippingDkk>,

    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<CreateTerminalConfigurationTippingEur>,

    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<CreateTerminalConfigurationTippingGbp>,

    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<CreateTerminalConfigurationTippingHkd>,

    /// Tipping configuration for JPY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jpy: Option<CreateTerminalConfigurationTippingJpy>,

    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<CreateTerminalConfigurationTippingMyr>,

    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<CreateTerminalConfigurationTippingNok>,

    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<CreateTerminalConfigurationTippingNzd>,

    /// Tipping configuration for PLN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pln: Option<CreateTerminalConfigurationTippingPln>,

    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<CreateTerminalConfigurationTippingSek>,

    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<CreateTerminalConfigurationTippingSgd>,

    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<CreateTerminalConfigurationTippingUsd>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationVerifoneP400 {

    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationWifi {

    /// Credentials for a WPA-Enterprise WiFi network using the EAP-PEAP authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_peap: Option<CreateTerminalConfigurationWifiEnterpriseEapPeap>,

    /// Credentials for a WPA-Enterprise WiFi network using the EAP-TLS authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_tls: Option<CreateTerminalConfigurationWifiEnterpriseEapTls>,

    /// Credentials for a WPA-Personal WiFi network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_psk: Option<CreateTerminalConfigurationWifiPersonalPsk>,

    /// Security type of the WiFi network.
    ///
    /// Fill out the hash with the corresponding name to provide the set of credentials for this security type.
    #[serde(rename = "type")]
    pub type_: CreateTerminalConfigurationWifiType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingAud {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingCad {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingChf {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingCzk {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingDkk {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingEur {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingGbp {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingHkd {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingJpy {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingMyr {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingNok {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingNzd {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingPln {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingSek {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingSgd {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationTippingUsd {

    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationWifiEnterpriseEapPeap {

    /// A File ID representing a PEM file containing the server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,

    /// Password for connecting to the WiFi network.
    pub password: String,

    /// Name of the WiFi network.
    pub ssid: String,

    /// Username for connecting to the WiFi network.
    pub username: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationWifiEnterpriseEapTls {

    /// A File ID representing a PEM file containing the server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_file: Option<String>,

    /// A File ID representing a PEM file containing the client certificate.
    pub client_certificate_file: String,

    /// A File ID representing a PEM file containing the client RSA private key.
    pub private_key_file: String,

    /// Password for the private key file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_file_password: Option<String>,

    /// Name of the WiFi network.
    pub ssid: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateTerminalConfigurationWifiPersonalPsk {

    /// Password for connecting to the WiFi network.
    pub password: String,

    /// Name of the WiFi network.
    pub ssid: String,
}

/// An enum representing the possible values of an `CreateTerminalConfigurationWifi`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateTerminalConfigurationWifiType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
}

impl CreateTerminalConfigurationWifiType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateTerminalConfigurationWifiType::EnterpriseEapPeap => "enterprise_eap_peap",
            CreateTerminalConfigurationWifiType::EnterpriseEapTls => "enterprise_eap_tls",
            CreateTerminalConfigurationWifiType::PersonalPsk => "personal_psk",
        }
    }
}

impl AsRef<str> for CreateTerminalConfigurationWifiType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTerminalConfigurationWifiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateTerminalConfigurationWifiType {
    fn default() -> Self {
        Self::EnterpriseEapPeap
    }
}

/// An enum representing the possible values of an `TerminalConfigurationConfigurationResourceWifiConfig`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TerminalConfigurationConfigurationResourceWifiConfigType {
    EnterpriseEapPeap,
    EnterpriseEapTls,
    PersonalPsk,
}

impl TerminalConfigurationConfigurationResourceWifiConfigType {
    pub fn as_str(self) -> &'static str {
        match self {
            TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapPeap => "enterprise_eap_peap",
            TerminalConfigurationConfigurationResourceWifiConfigType::EnterpriseEapTls => "enterprise_eap_tls",
            TerminalConfigurationConfigurationResourceWifiConfigType::PersonalPsk => "personal_psk",
        }
    }
}

impl AsRef<str> for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TerminalConfigurationConfigurationResourceWifiConfigType {
    fn default() -> Self {
        Self::EnterpriseEapPeap
    }
}
