//! HTML rendering

use crate::types::*;
use horrorshow::helper::doctype;
use horrorshow::{html, Raw};

/// # Render assets to HTML
///
/// Create a self-contained HTML document that portrays all the specified
/// assets, along with click-through links to the offical iCloud Photo shared
/// library webpage.
///
/// The images are linked directly from the iCloud website, to avoid bloating an
/// HTML email, and avoid issues with multiple image embedding in some email
/// clients. The one embedded image (a play button to disambiguate videos) is
/// included as base64 CSS, to avoid any attachment requirements.
pub fn build(
    config: &Config,
    assets: Vec<&Asset>,
    thumbnail_urls: ThumbnailLocations,
) -> String {
    format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title : format!("New {} photos", config.album_name);
                    style : css();
                }
                body {
                    p(class="emph") {
                        : format!("There are {} new photos available in ", assets.len());
                        a(href = &config.album_id.url()) {
                            : format!("your {} shared photo album", config.album_name)
                        }
                        : "."
                     }
                     p { :"You may be able to see some small blurry versions below,
                           depending on your email app's security preferences. Whether
                           you can, or just see empty boxes, please click on the link above,
                           or one of the pictures below, to see the photos or videos at
                           full resolution."
                    }
                    br;
                    div(class = "container") {
                        @ for asset in &assets {
                            a(href = &config.album_id.asset_url(&asset.guid)) {
                                img(width  = asset.width,
                                    height = asset.height,
                                    src    = &thumbnail_urls.get(&asset.checksum).unwrap().0);
                                @ if asset.asset_type == AssetType::Video {
                                    div(class = "play-button")
                                }
                            }
                        }
                    }
                    br;
                }
            }
        }
    )
}

/// Embedded CSS.
///
/// Uses `Raw` to avoid HTML escaping of eg "quotes"
fn css() -> Raw<String> {
    let s = r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, Roboto, sans-serif;
            text-align: center;
        }
        div.container {
            display: flex;
            flex-wrap: wrap;
            justify-content: space-around;
            align-items: center;
            align-content: center;
        }
        .emph {
            font-weight: bold;
            font-size: 120%;
        }
        a {
            position: relative;
        }
        img {
            margin: 5px;
            border: solid 1px black;
        }
        div.play-button {
            position: absolute;
            left: 50%;
            top: 50%;
            margin-left: -36px;
            margin-top: -36px;
            width: 73px;
            height: 73px;
            z-index: 1;
            background-size: 73px 73px;
            background-image: url(data:image/png;base64,%%%);
        }"#
    .to_string()
    .replace("%%%", PLAY_PNG);

    // Cheesy CSS minifier
    Raw(s.lines().map(str::trim_start).collect())
}

/// The iCloud "video play button" png (73x73px @2), base64 encoded
static PLAY_PNG: &str = "\
    iVBORw0KGgoAAAANSUhEUgAAAJIAAACSCAQAAAAEclsFAAAACXBIWXMAAAsTAAALEwEAmpwYAAAF\
    FWlUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0w\
    TXBDZWhpSHpyZVN6TlRjemtjOWQiPz4gPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRh\
    LyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgNi4wLWMwMDIgNzkuMTY0NDYwLCAyMDIwLzA1LzEy\
    LTE2OjA0OjE3ICAgICAgICAiPiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3Jn\
    LzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPiA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0i\
    IiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtbG5zOmRjPSJodHRw\
    Oi8vcHVybC5vcmcvZGMvZWxlbWVudHMvMS4xLyIgeG1sbnM6cGhvdG9zaG9wPSJodHRwOi8vbnMu\
    YWRvYmUuY29tL3Bob3Rvc2hvcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNv\
    bS94YXAvMS4wL21tLyIgeG1sbnM6c3RFdnQ9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9z\
    VHlwZS9SZXNvdXJjZUV2ZW50IyIgeG1wOkNyZWF0b3JUb29sPSJBZG9iZSBQaG90b3Nob3AgMjEu\
    MiAoTWFjaW50b3NoKSIgeG1wOkNyZWF0ZURhdGU9IjIwMjAtMDctMjFUMTI6NTM6MTkrMDE6MDAi\
    IHhtcDpNb2RpZnlEYXRlPSIyMDIwLTA3LTIxVDIyOjAyOjIzKzAxOjAwIiB4bXA6TWV0YWRhdGFE\
    YXRlPSIyMDIwLTA3LTIxVDIyOjAyOjIzKzAxOjAwIiBkYzpmb3JtYXQ9ImltYWdlL3BuZyIgcGhv\
    dG9zaG9wOkNvbG9yTW9kZT0iMSIgcGhvdG9zaG9wOklDQ1Byb2ZpbGU9IkRvdCBHYWluIDIwJSIg\
    eG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDo1MDc2ZTk1OC0wOWY3LTRkMmQtYjgyZC02ZDk3MWNh\
    NjRkMzQiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6NTA3NmU5NTgtMDlmNy00ZDJkLWI4MmQt\
    NmQ5NzFjYTY0ZDM0IiB4bXBNTTpPcmlnaW5hbERvY3VtZW50SUQ9InhtcC5kaWQ6NTA3NmU5NTgt\
    MDlmNy00ZDJkLWI4MmQtNmQ5NzFjYTY0ZDM0Ij4gPHhtcE1NOkhpc3Rvcnk+IDxyZGY6U2VxPiA8\
    cmRmOmxpIHN0RXZ0OmFjdGlvbj0iY3JlYXRlZCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDo1\
    MDc2ZTk1OC0wOWY3LTRkMmQtYjgyZC02ZDk3MWNhNjRkMzQiIHN0RXZ0OndoZW49IjIwMjAtMDct\
    MjFUMTI6NTM6MTkrMDE6MDAiIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCAy\
    MS4yIChNYWNpbnRvc2gpIi8+IDwvcmRmOlNlcT4gPC94bXBNTTpIaXN0b3J5PiA8L3JkZjpEZXNj\
    cmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PhqmUIkA\
    ABYRSURBVHic7V17XFTV2n42Aw0YN2EIARXUQkFFVDDFI6iRFwpN1KysvETqsctXaWVyjqfPcyzN\
    7OvrWKmRR0szb6ThXdLUAuWSiBcUIxAFLwwIDgcYZ3B/f+wLs9fec98zg+f3Pf+wZu29117rYa93\
    rfWud70vReP/YQ7uAOXqOjwIbwBAINwNcvWoAwA04d8uqJMBaEG1nAcKDyEAvugMFUGNFPSogxq3\
    cQf1uAUXfPoU7cwvSYGu6IYwdLWjjGpcw1VcQ5tstTID2hhJcv+7qC7oga4Ig0KmAttQjWuooG/I\
    VB4BQ06cQBLli76Ihr/xO0KUo4MAIC7Yy6M9t0VXeBMAjtRe15oovgEXcJ6+I0dNDeE0kiglIhEt\
    3bXGB8U/1CugV3C3kKBAL2/T5TRp6uuvXi+/WV5fcGt/reQt13ABZbQpMq2EU0ii/DAE0WKRnBTw\
    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\
    pEHJ3bH+E/F/CZ+cmLJGKw8AAAAASUVORK5CYII=";
