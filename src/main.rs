#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Check a public iCloud Shared Photo library, and if any new photos/videos
//! have been posted since last time we ran, send an email summarising the
//! new content.

mod email;
mod fetch;
mod html;
mod types;
mod utils;

use clap::Clap;
use std::collections::HashSet;
use types::*;
use utils::OrDie;

/// Poll an iCloud Shared Photo library and email any updates
#[derive(Clap, Debug)]
#[clap(version = "1.0.0")]
pub struct Opts {
    /// Mandatory JSON configuration file
    #[clap(short, long)]
    pub config: String,
}

/// Overall program logic:
///
///  - Load the required config JSON file
///  - Load all the local state
///  - Fetch the state from iCloud
///  - If there are new photos in iCloud, get all the required info (thumbnail
///    URL + size, click-through URL) and compose an HTML document displaying it
///  - Send an email
///  - Update the local state for which photo/video assets have been seen
fn main() {
    // Parse command-line options
    let opts = Opts::parse();

    // Load config.
    let config: Config = utils::load_json(&opts.config)
        .or_die(format!("successfully parse file {}", opts.config));

    // Load the database of previously seen Guids if available.
    let seen_guids: HashSet<Guid> = utils::load_json(&config.db_file).unwrap_or_default();

    // Fetch all available assets from iCloud.
    let all_assets = fetch::all_assets(&config.album_id)
        .or_die(format!("download {}", config.album_id));

    // Just the Guids, indexed for lookup
    let new_guid_set: HashSet<&Guid> = all_assets.iter().map(|a| &a.guid).collect();

    // Minority case: see if any previously-seen assets have disappeared.
    seen_guids
        .iter()
        .filter(|old_guid| !new_guid_set.contains(old_guid))
        .for_each(|guid| {
            eprintln!("Warning: DB has seen {} but this has disappeared", guid)
        });

    // Mainline case: see which assets have not been previously seen. We've
    // carefully preserved order, so that if there are any of these, they are in
    // the same order as on the iCloud site.
    let new_assets: Vec<_> = all_assets
        .iter()
        .filter(|asset| !seen_guids.contains(&asset.guid))
        .collect();

    // If there's nothing to do, then just exit now
    let num_new_assets = new_assets.len();
    if num_new_assets == 0 {
        return;
    }

    // Fetch thumbnail URLs for all the new assets.
    let new_guids: Vec<&Guid> = new_assets.iter().map(|a| &a.guid).collect();
    let thumbnail_urls = fetch::thumbnail_urls(&new_guids, &config)
        .or_die(format!("fetch data for {} new guids", new_guids.len()));

    // Build the HTML for all new things.
    let html = html::build(&config, new_assets, thumbnail_urls);

    // Send it over email
    email::send(&config, html).or_die("send email");
    println!("Sent email for {} new assets", num_new_assets);

    // Update the database of seen Guids.
    let all_guids: Vec<&Guid> = all_assets.iter().map(|a| &a.guid).collect();
    utils::save_json(&all_guids, &config.db_file)
        .or_die(format!("save file {}", config.db_file));
}
