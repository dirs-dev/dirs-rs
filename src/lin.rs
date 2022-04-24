extern crate dirs_sys;

use std::env;
use std::path::PathBuf;

pub use crate::xdg_basedir::{
    cache_dir, config_dir, data_dir, data_local_dir, executable_dir, preference_dir, runtime_dir,
    state_dir,
};

pub fn home_dir()       -> Option<PathBuf> { dirs_sys::home_dir() }

pub fn audio_dir()      -> Option<PathBuf> { dirs_sys::user_dir("MUSIC") }
pub fn desktop_dir()    -> Option<PathBuf> { dirs_sys::user_dir("DESKTOP") }
pub fn document_dir()   -> Option<PathBuf> { dirs_sys::user_dir("DOCUMENTS") }
pub fn download_dir()   -> Option<PathBuf> { dirs_sys::user_dir("DOWNLOAD") }
pub fn font_dir()       -> Option<PathBuf> { data_dir().map(|d| d.join("fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { dirs_sys::user_dir("PICTURES") }
pub fn public_dir()     -> Option<PathBuf> { dirs_sys::user_dir("PUBLICSHARE") }
pub fn template_dir()   -> Option<PathBuf> { dirs_sys::user_dir("TEMPLATES") }
pub fn video_dir()      -> Option<PathBuf> { dirs_sys::user_dir("VIDEOS") }

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_user_dirs_exists() {
        let user_dirs_file = ::config_dir().unwrap().join("user-dirs.dirs");
        println!("{:?} exists: {:?}", user_dirs_file, user_dirs_file.exists());
    }
}
