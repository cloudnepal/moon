use schematic::{validate, Config};

/// Configures how and where updates will be received.
#[derive(Clone, Config, Debug)]
pub struct MoonConfig {
    /// A secure URL to lookup the latest version.
    #[setting(validate = validate::url_secure, default = "https://launch.moonrepo.app/versions/cli/current")]
    pub manifest_url: String,

    /// A secure URL for downloading the moon binary.
    #[setting(validate = validate::url_secure, default = "https://github.com/moonrepo/moon/releases/latest/download")]
    pub download_url: String,
}