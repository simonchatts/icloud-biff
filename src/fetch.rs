//! Fetch data from iCloud

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Convenience type for errors.
type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

///////////////////////////////////////////////////////////////////////////////
///
/// Synchronously fetch all the assets available from the iCloud photo album.
pub fn all_assets(album_id: &AlbumId) -> Result<Vec<Asset>, AnyError> {
    let post_data = serde_json::json!({ "streamCtag": null });
    // We use an async http client, but just block on it straight away...
    let result = smol::block_on(
        surf::post(album_id.all_assets())
            .body_json(&post_data)?
            .recv_json::<AllAssetResponse>(),
    )?
    // ...and then process the result.
    .photos
    .into_iter()
    .map(process)
    .collect();
    Ok(result)
}

//
// Types corresponding to JSON values on the wire
//

/// Response container
#[derive(Debug, Deserialize)]
struct AllAssetResponse {
    photos: Vec<RawAsset>,
}

/// Per-asset data.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawAsset {
    photo_guid: Guid,
    // Map detailing each available resolution of this asset. The keys are the pixel
    // count of longest side, as a stringified int (obv).
    derivatives: HashMap<String, RawAssetSize>,

    // This is omitted for the default Photo type, and "video" for Video.
    #[serde(default)]
    media_asset_type: AssetType,
}

/// Per-asset-at-a-particular-resolution data.
#[derive(Debug, Deserialize)]
struct RawAssetSize {
    width: String,
    height: String,
    checksum: Checksum,
}

/// Parse the externally-defined data into our preferred internal format.
fn process(photo: RawAsset) -> Asset {
    // Choose the best thumbnail image from all the alternatives.
    let thumbnail = if photo.media_asset_type == AssetType::Video {
        // Video: there's a handy "PosterFrame" provided.
        photo.derivatives.get("PosterFrame").unwrap()
    } else {
        // Photo: choose the smallest size available (specified as keys,
        // that are stringified integers specifying max(width, height)).
        let mut keys: Vec<_> = photo.derivatives.keys().collect();
        keys.sort_by_cached_key(|s| s.parse::<i32>().expect("photo size"));
        photo.derivatives.get(keys[0]).unwrap()
    };

    // Video thumbnails are disproportionately large vs photo ones.
    let shrink = |x: u16| -> u16 {
        match photo.media_asset_type {
            AssetType::Photo => x / 2,
            AssetType::Video => x / 6,
        }
    };

    // Create internal representation.
    Asset {
        guid: photo.photo_guid.clone(),
        asset_type: photo.media_asset_type,
        checksum: thumbnail.checksum.clone(),
        width: shrink(thumbnail.width.parse().expect("width")),
        height: shrink(thumbnail.height.parse().expect("height")),
    }
}

///////////////////////////////////////////////////////////////////////////////
///
/// Synchronously fetch the thumbnail data for the specified Guids.
pub fn thumbnail_urls(
    photo_guids: &[&Guid],
    config: &Config,
) -> Result<HashMap<Checksum, URL>, AnyError> {
    // We use an async http client, but just block on it straight away...
    let result = smol::block_on(
        surf::post(config.album_id.asset_urls())
            .body_json(&FetchThumbnailsRequest { photo_guids })?
            .recv_json::<FetchThumbnailsResponse>(),
    )?
    // ...and then process the result.
    .items
    .into_iter()
    .map(|(k, v)| (k, v.as_url()))
    .collect();
    Ok(result)
}

//
// Types corresponding to the externally-defined JSON format
//

/// Request body, to get URLS for a set of assets.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FetchThumbnailsRequest<'a> {
    photo_guids: &'a [&'a Guid],
}

/// Request response, mapping Checksum (identifying asset + resolution) to
/// Location.
#[derive(Debug, Deserialize)]
struct FetchThumbnailsResponse {
    items: HashMap<Checksum, Location>,
}

/// Location of a single asset at a particular resolution.
#[derive(Debug, Deserialize)]
struct Location {
    url_location: String,
    url_path: String,
}

impl Location {
    /// Render a Location as a standard URL. See `PROTOCOL.md` - technically
    /// there is more to this, but in practice, this seems all that's required.
    fn as_url(&self) -> URL {
        URL(format!("https://{}{}", self.url_location, self.url_path))
    }
}
