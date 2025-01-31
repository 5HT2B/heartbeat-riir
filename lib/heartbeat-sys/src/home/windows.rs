use std::{env, path::PathBuf};

pub fn home_dir_inner() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(home_dir_crt)
}

#[cfg(not(target_vendor = "uwp"))]
fn home_dir_crt() -> Option<PathBuf> {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};

    use windows_sys::Win32::{
        Foundation::{MAX_PATH, S_OK},
        UI::Shell::{SHGetFolderPathW, CSIDL_PROFILE},
    };

    unsafe {
        let mut path = Vec::with_capacity(MAX_PATH as usize);
        #[allow(clippy::cast_possible_wrap)] // CSIDL_PROFILE is just 40.
        match SHGetFolderPathW(0, CSIDL_PROFILE as i32, 0, 0, path.as_mut_ptr()) {
            S_OK => {
                let len = wcslen(path.as_ptr());
                path.set_len(len);
                let s = OsString::from_wide(&path);
                Some(PathBuf::from(s))
            }
            _ => None,
        }
    }
}

#[cfg(target_vendor = "uwp")]
fn home_dir_crt() -> Option<PathBuf> {
    None
}

extern "C" {
    fn wcslen(buf: *const u16) -> usize;
}

#[cfg(not(target_vendor = "uwp"))]
#[cfg(test)]
mod tests {
    use super::home_dir_inner;
    use std::{
        env,
        path::{Path, PathBuf},
    };

    #[test]
    fn test_with_without() {
        let old_userprofile = env::var_os("USERPROFILE").unwrap_or_default();
        env::remove_var("HOME");
        env::remove_var("USERPROFILE");
        assert_eq!(home_dir_inner(), Some(PathBuf::from(old_userprofile)));
        let home = Path::new(r"C:\Users\blon fel fotch passameer day slitheen");
        env::set_var("HOME", home.as_os_str());
        assert_ne!(home_dir_inner().as_deref(), Some(home));
        env::set_var("USERPROFILE", home.as_os_str());
        assert_eq!(home_dir_inner().as_deref(), Some(home));
    }
}
