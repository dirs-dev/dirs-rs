#![cfg(target_os = "windows")]

use std;
use std::path::PathBuf;

extern crate winapi;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winnt;

pub fn home_dir()       -> PathBuf { unsafe { known_folder(&knownfolders::FOLDERID_Profile) } }
pub fn data_dir()       -> PathBuf { unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) } }
pub fn data_local_dir() -> PathBuf { unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) } }
pub fn cache_dir()      -> PathBuf { data_local_dir() }
pub fn config_dir()     -> PathBuf { data_dir() }
pub fn executable_dir() -> Option<PathBuf> { None }
pub fn runtime_dir()    -> Option<PathBuf> { None }
pub fn audio_dir()      -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Music) }) }
pub fn desktop_dir()    -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Desktop) }) }
pub fn document_dir()   -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Documents) }) }
pub fn download_dir()   -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Downloads) }) }
pub fn font_dir()       -> Option<PathBuf> { None }
pub fn picture_dir()    -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Pictures) }) }
pub fn public_dir()     -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Public) }) }
pub fn template_dir()   -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Templates) }) }
    // see https://github.com/soc/directories-rs/issues/18
    // let trash_dir      = unsafe { known_folder(&knownfolders::FOLDERID_RecycleBinFolder) };
pub fn video_dir()      -> Option<PathBuf> { Some(unsafe { known_folder(&knownfolders::FOLDERID_Videos) }) }

unsafe fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> PathBuf {
    let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
    let _result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
    let len = length_of_u16_string(path_ptr);
    let path = std::slice::from_raw_parts(path_ptr, len);
    let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
    combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
    PathBuf::from(ostr)
}

unsafe fn length_of_u16_string(ptr: *mut u16) -> usize {
    let mut index = 0;
    while *ptr.offset(index as isize) != 0 as u16 {
        index += 1;
    }
    index
}
