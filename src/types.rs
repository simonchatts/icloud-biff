//! Domain types for iCloud-biff
use derivative::Derivative;
use derive_more::Display;
use serde::{Deserialize, Serialize};

//////////////////////////////////////////////////////////////////////////////
///
/// # User-supplied configuration.
///
/// The `album_name` is title for humans (eg "My lovely dogs"). The `album_id`
/// is the identifier for iCloud, eg "B0zAxqIORGhwx3u".
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub album_name: String,
    pub album_id: AlbumId,
    pub recipient_email_addrs: Vec<String>,
    pub sender_email_addr: String,
    pub sender_email_name: String,
    pub db_file: String,
    #[serde(default = "default_sendmail_path")]
    pub sendmail_path: String,
}

fn default_sendmail_path() -> String { "/usr/sbin/sendmail".to_string() }

//////////////////////////////////////////////////////////////////////////////
///
// Basic newtypes

/// A Guid identifying a particuar asset.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub struct Guid(String);

/// A checksum identifying an asset at a particular resolution.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checksum(String);

/// A URL. Insides are public for easy rendering into HTML.
#[derive(Debug, Clone, Deserialize)]
pub struct Url(pub String);

/// An album id
#[derive(Debug, Serialize, Deserialize, Display)]
pub struct AlbumId(String);

impl AlbumId {
    // User-facing URL to view the whole album.
    pub fn url(&self) -> String {
        format!("https://www.icloud.com/sharedalbum/#{}", self)
    }

    // User-facing URL to view a single asset in the album.
    pub fn asset_url(&self, guid: &Guid) -> String {
        format!("https://www.icloud.com/sharedalbum/#{};{}", self, guid)
    }

    /// URL for a JSON document listing all photo/video assets in this album.
    pub fn all_assets(&self) -> String {
        format!(
            "https://p37-sharedstreams.icloud.com/{}/sharedstreams/webstream",
            self
        )
    }

    /// URL for a JSON document listing URLS for a set of specified assets.
    pub fn asset_urls(&self) -> String {
        format!(
            "https://p37-sharedstreams.icloud.com/{}/sharedstreams/webasseturls",
            self
        )
    }
}

//////////////////////////////////////////////////////////////////////////////
///
/// # Internal definition of a photo or video asset.
///
/// The `guid` identifies the asset, which can have multiple instantiations at
/// different resolutions. The `checksum` specifically identfies the best
/// instantiation of that asset for a thumbnail, with recommended dimensions
/// `width`x`height` px.
#[derive(Debug)]
pub struct Asset {
    pub guid: Guid,
    pub asset_type: AssetType,
    pub checksum: Checksum,
    pub width: u16,
    pub height: u16,
}
/// Asset type: photo or video.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Default)]
pub enum AssetType {
    // In the JSON iCloud encoding, the default (absent field) means photo
    #[derivative(Default)]
    Photo,
    Video,
}
