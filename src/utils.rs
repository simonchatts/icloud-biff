//! Misc utils

use async_std::fs;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;

//////////////////////////////////////////////////////////////////////////////
///
/// # Basic error handling helper
///
/// A use case such as this:
///
///   result.or_die("defuzz the frobnicator")
///
/// turns a `Result<T, E>` into a `T`, aborting the process with an error
/// message in the error case. The error message is more user-friendly than a
/// `panic`, eg:
///
///   Unable to defuzz the frobnicator: Incompatible spanner
///
/// The message is convertible into a real `String` because the mainline case is
/// it already is (via `format!`) and if not, one more allocation is not a
/// problem.
pub trait OrDie<S, T>
where
    S: Into<String>,
{
    fn or_die(self, msg: S) -> T;
}

/// Implement `OrDie` for `Result` types (where error is `Display`able, as
/// usual).
impl<S, T, E> OrDie<S, T> for Result<T, E>
where
    S: Into<String>,
    E: Display,
{
    fn or_die(self, msg: S) -> T {
        self.unwrap_or_else(|err| {
            eprintln!("Unable to {}: {}", msg.into(), err);
            std::process::exit(1);
        })
    }
}

//////////////////////////////////////////////////////////////////////////////
//
// JSON load from/save to file

/// Load a JSON file.
pub async fn load_json<T>(fname: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let raw_json = fs::read_to_string(fname).await?;
    serde_json::from_str(&raw_json).map_err(|e| e.into())
}

/// Save a JSON file.
pub async fn save_json<T>(this: &T, fname: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let raw_json = serde_json::to_vec_pretty(this)?;
    fs::write(fname, raw_json).await.map_err(|e| e.into())
}
