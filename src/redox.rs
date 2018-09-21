use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::str;

extern crate syscall;

pub fn home_dir() -> Option<PathBuf> {
    let current_uid = syscall::getuid().ok()?;

    let mut reader = File::open("/etc/passwd").ok().map(BufReader::new)?;

    let mut entry = Vec::new();
    while { entry.clear(); reader.read_until(b'\n', &mut entry).ok()? > 0 } {
        if entry.split(|b| *b == b';').count() != 6 {
            // Invalid entry!
            continue;
        }
        let mut parts = entry.split(|b| *b == b';');

        /* name */   parts.next().unwrap();
        let uid: usize = match str::from_utf8(parts.next().unwrap()).ok().and_then(|s| s.parse().ok()) {
            Some(uid) => uid,
            None => continue
        };
        /* gid */    parts.next().unwrap();
        /* gecos */  parts.next().unwrap();
        let home =   parts.next().unwrap();

        if uid == current_uid {
            return Some(PathBuf::from(OsString::from_vec(home.to_vec())));
        }
    }
    None
}
pub fn cache_dir()      -> Option<PathBuf> { env::var_os("XDG_CACHE_HOME") .and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".cache"))) }
pub fn config_dir()     -> Option<PathBuf> { env::var_os("XDG_CONFIG_HOME").and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".config"))) }
pub fn data_dir()       -> Option<PathBuf> { env::var_os("XDG_DATA_HOME")  .and_then(is_absolute_path).or_else(|| home_dir().map(|h| h.join(".local/share"))) }
pub fn data_local_dir() -> Option<PathBuf> { data_dir().clone() }
pub fn runtime_dir()    -> Option<PathBuf> { env::var_os("XDG_RUNTIME_DIR").and_then(is_absolute_path) }
pub fn executable_dir() -> Option<PathBuf> {
    env::var_os("XDG_BIN_HOME").and_then(is_absolute_path).or_else(|| {
        data_dir().map(|mut e| { e.pop(); e.push("bin"); e })
    })
}
pub fn audio_dir()      -> Option<PathBuf> { home_dir().map(|h| h.join("Music")) }
pub fn desktop_dir()    -> Option<PathBuf> { home_dir().map(|h| h.join("Desktop")) }
pub fn document_dir()   -> Option<PathBuf> { home_dir().map(|h| h.join("Documents")) }
pub fn download_dir()   -> Option<PathBuf> { home_dir().map(|h| h.join("Downloads")) }
pub fn font_dir()       -> Option<PathBuf> { data_dir().map(|d| d.join("fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { home_dir().map(|h| h.join("Pictures")) }
pub fn public_dir()     -> Option<PathBuf> { home_dir().map(|h| h.join("Public")) }
pub fn template_dir()   -> Option<PathBuf> { home_dir().map(|h| h.join("Templates")) }
pub fn video_dir()      -> Option<PathBuf> { home_dir().map(|h| h.join("Videos")) }

// we don't need to explicitly handle empty strings in the code above,
// because an empty string is not considered to be a absolute path here.
fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}
