#![cfg(target_os = "linux")]

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

pub fn home_dir()       -> PathBuf { env::home_dir().unwrap() }
pub fn cache_dir()      -> PathBuf { env::var_os("XDG_CACHE_HOME") .and_then(is_absolute_path).unwrap_or_else(|| home_dir().join(".cache")) }
pub fn config_dir()     -> PathBuf { env::var_os("XDG_CONFIG_HOME").and_then(is_absolute_path).unwrap_or_else(|| home_dir().join(".config")) }
pub fn data_dir()       -> PathBuf { env::var_os("XDG_DATA_HOME")  .and_then(is_absolute_path).unwrap_or_else(|| home_dir().join(".local/share")) }
pub fn data_local_dir() -> PathBuf { data_dir().clone() }
pub fn runtime_dir()    -> Option<PathBuf> { env::var_os("XDG_RUNTIME_DIR").and_then(is_absolute_path) }
pub fn executable_dir() -> Option<PathBuf> {
    let exec_dir = env::var_os("XDG_BIN_HOME").and_then(is_absolute_path).unwrap_or_else(|| {
        let mut new_dir = data_dir(); new_dir.pop(); new_dir.push("bin"); new_dir });
    Some(exec_dir)
}
pub fn audio_dir()      -> Option<PathBuf> { run_xdg_user_dir_command("MUSIC") }
pub fn desktop_dir()    -> Option<PathBuf> { run_xdg_user_dir_command("DESKTOP") }
pub fn document_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("DOCUMENTS") }
pub fn download_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("DOWNLOAD") }
pub fn font_dir()       -> Option<PathBuf> { Some(data_dir().join("fonts")) }
pub fn picture_dir()    -> Option<PathBuf> { run_xdg_user_dir_command("PICTURES") }
pub fn public_dir()     -> Option<PathBuf> { run_xdg_user_dir_command("PUBLICSHARE") }
pub fn template_dir()   -> Option<PathBuf> { run_xdg_user_dir_command("TEMPLATES") }
pub fn video_dir()      -> Option<PathBuf> { run_xdg_user_dir_command("VIDEOS") }

fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

fn run_xdg_user_dir_command(arg: &str) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    let mut out  = Command::new("xdg-user-dir").arg(arg).output().ok()?.stdout;
    let out_len = out.len();
    out.truncate(out_len - 1);
    Some(PathBuf::from(OsString::from_vec(out)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_user_dirs_exists() {
        let user_dirs_file = ::config_dir().join("user-dirs.dirs");
        println!("{:?} exists: {:?}", user_dirs_file, user_dirs_file.exists());
    }
}
