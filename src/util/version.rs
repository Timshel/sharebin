extern crate reqwest;
extern crate serde;
extern crate serde_json;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
}

pub static CURRENT_VERSION: Lazy<Version> = Lazy::new(|| Version {
    name: "2.0.4".to_string(),
    draft: false,
    prerelease: false,
});

impl Version {
    pub fn newer_than(&self, other: &Version) -> bool {
        return self.name != other.name
    }

    pub fn newer_than_current(&self) -> bool {
        self.newer_than(&CURRENT_VERSION)
    }
}

pub async fn fetch_latest_version() -> Option<Version> {
    let url = "https://api.github.com/repos/timshel/sharebin/releases?per_page=1";

    match reqwest::get(url).await.ok() {
        Some(response) => response.json::<Vec<Version>>().await.ok(),
        None           => None,
    }.and_then(|versions| { versions.into_iter().next() } )
}
