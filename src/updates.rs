//! Self-update from GitHub releases, through fastframe-update.
//!
//! The crate checks for a newer release, refuses package-managed copies,
//! downloads and verifies the update against the publisher signature, and
//! hands it to a helper that installs it and rolls back if it does not start.
//! Zap Faster keeps its names, its key, its proxy and its interface.

pub use fastframe_update::{
    CHECK_INTERVAL, DownloadState, Installation, Kind, Prepared, Release, Source, Unsupported,
    Updater,
};
use fastframe_update::{MacConfig, ReqwestTransport, UpdateConfig};

pub use crate::config::GITHUB_REPO_URL;

/// ZapFastest's releases and the names its installations have had.
pub const CONFIG: UpdateConfig = UpdateConfig {
    // Upstream installations are never treated as ZapFastest update targets.
    legacy_names: &["zap-faster"],
    macos: MacConfig {
        bundle_ids: &[
            "io.github.eternalwaitt.ZapFastest",
            "io.github.eternalwaitt.ZapFaster",
        ],
        executable_names: &["zap-fastest", "zap-faster"],
        legacy_bundle_names: &["Zap Faster", "ZapFaster"],
    },
    publisher_key: Some(include_str!("../assets/update-public-key.hex")),
    // Only ZapFastest signatures authorize updates.
    additional_publisher_keys: &[],
    ..UpdateConfig::new(
        crate::config::REPOSITORY,
        "ZapFastest",
        "zap-fastest",
        env!("CARGO_PKG_VERSION"),
    )
};

/// An updater on the proxy-aware reqwest client.
pub fn updater() -> anyhow::Result<Updater> {
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(proxy) = crate::proxy::reqwest_proxy() {
        builder = builder.proxy(proxy);
    }
    Ok(Updater::new(CONFIG, ReqwestTransport::new(builder)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_config_is_valid() {
        CONFIG.validate().unwrap();
        assert_eq!(CONFIG.current_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(CONFIG.slug, "zap-fastest");
        assert_eq!(CONFIG.repository, crate::config::REPOSITORY);
        assert!(CONFIG.additional_publisher_keys.is_empty());
        assert_eq!(CONFIG.legacy_names, &["zap-faster"]);
    }

    #[test]
    fn the_updater_starts_on_github() {
        assert!(updater().unwrap().source().is_github());
    }
}
