//! The `xdg_basedir` module respects the [XDG base directory](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
//! on both Linux and macOS, even if the environment variables are not set.
//!
//! It is not recommended to use it for general-purpose applications, especially those that are mostly configured
//! through a GUI. On the other hand, it is recommended to use for developer-focused tools since lots of developers
//! will expect their configuration files for such tools to end up in `XDG_CONFIG_HOME`, with `$HOME/.config` as a
//! default if it is not set.
//!
//! Note: the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) is **not** respected here,
//! platform directories are still used, only the _base directory_ part of the specification is used.
//!
//! # Usage
//!
//! For convenience, the `xdg_basedir` module re-exports everything it does not implement from the root of the crate.
//! Using this module transparently can then be done through the use of `cfg`:
//!
//! ```no_run
//! #[cfg(not(any(target_os = "windows", target_os = "ios", target_arch = "wasm32")))]
//! use dirs::xdg_basedir as dirs;
//!
//! // Use dirs:: as you would normally
//! ```
use std::env;

pub use super::*;

/// `$XDG_CACHE_HOME`, defaults to `$HOME/.cache`
pub fn cache_dir()      -> Option<PathBuf> { env::var_os("XDG_CACHE_HOME").and_then(dirs_sys::is_absolute_path).or_else(|| home_dir().map(|h| h.join(".cache"))) }
/// `$XDG_CONFIG_HOME`, defaults to `$HOME/.config`
pub fn config_dir()     -> Option<PathBuf> { env::var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path).or_else(|| home_dir().map(|h| h.join(".config"))) }
/// `$XDG_DATA_HOME`, defaults to `$HOME/.local/share`
pub fn data_dir()       -> Option<PathBuf> { env::var_os("XDG_DATA_HOME").and_then(dirs_sys::is_absolute_path).or_else(|| home_dir().map(|h| h.join(".local/share"))) }
/// See [`data_dir()`]
pub fn data_local_dir() -> Option<PathBuf> { data_dir() }
/// See [`config_dir()`]
pub fn preference_dir() -> Option<PathBuf> { config_dir() }
/// `$XDG_RUNTIME_DIR`, defaults to nothing
pub fn runtime_dir()    -> Option<PathBuf> { env::var_os("XDG_RUNTIME_DIR").and_then(dirs_sys::is_absolute_path) }
/// `$XDG_STATE_HOME`, defaults to `$HOME/.local/state`
pub fn state_dir()      -> Option<PathBuf> { env::var_os("XDG_STATE_HOME").and_then(dirs_sys::is_absolute_path).or_else(|| home_dir().map(|h| h.join(".local/state"))) }
/// `$XDG_BIN_HOME`, defaults to `$HOME/.local/bin`
pub fn executable_dir() -> Option<PathBuf> { env::var_os("XDG_BIN_HOME").and_then(dirs_sys::is_absolute_path).or_else(|| home_dir().map(|h| h.join(".local/bin"))) }

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    #[test]
    #[cfg(target_os = "macos")]
    fn macos_cache_dir() {
        env::remove_var("XDG_CACHE_HOME");

        let default_path = dirs_sys::home_dir().map(|h| h.join(".cache")).unwrap();
        assert_eq!(super::cache_dir().unwrap(), default_path);

        env::set_var("XDG_CACHE_HOME", "/Users");
        assert_eq!(super::cache_dir().unwrap(), PathBuf::from("/Users"));
    }
}
