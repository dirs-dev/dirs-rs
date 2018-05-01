#![cfg(target_os = "macos")]

use std::env;
use std::path::PathBuf;

pub fn home_dir()       -> PathBuf { env::home_dir().unwrap() }
pub fn cache_dir()      -> PathBuf { home_dir().join("Library/Caches") }
pub fn config_dir()     -> PathBuf { home_dir().join("Library/Preferences") }
pub fn data_dir()       -> PathBuf { home_dir().join("Library/Application Support") }
pub fn data_local_dir() -> PathBuf { data_dir() }
pub fn executable_dir() -> Option<PathBuf> { None }
pub fn runtime_dir()    -> Option<PathBuf> { None }
pub fn audio_dir()      -> Option<PathBuf> { Some(home_dir().join("Music")) }
pub fn desktop_dir()    -> Option<PathBuf> { Some(home_dir().join("Desktop")) }
pub fn document_dir()   -> Option<PathBuf> { Some(home_dir().join("Documents")) }
pub fn download_dir()   -> Option<PathBuf> { Some(home_dir().join("Downloads")) }
pub fn font_dir()       -> Option<PathBuf> { Some(home_dir().join("Library/Fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { Some(home_dir().join("Pictures")) }
pub fn public_dir()     -> Option<PathBuf> { Some(home_dir().join("Public")) }
pub fn template_dir()   -> Option<PathBuf> { None }
// pub fn trash_dir() -> Option<PathBuf> { Some(home_dir().join(".trash")) }
pub fn video_dir()      -> Option<PathBuf> { Some(home_dir().join("Movies")) }
