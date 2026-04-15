#![cfg_attr(not(feature = "std"), no_std)]

use core::str::FromStr;

#[inline]
pub fn split_kv(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'=' {
            return Some((&s[..i], &s[i + 1..]));
        }
        i += 1;
    }

    None
}

#[inline]
pub fn parse<T: FromStr>(s: &str) -> Option<T> {
    s.parse().ok()
}

pub trait EnvBackend {
    fn get(key: &str) -> Option<&'static str>;
}

#[cfg(feature = "std")]
mod backend {
    use super::*;
    use libc::c_char;
    use std::collections::HashMap;
    use std::ffi::CStr;
    use std::sync::OnceLock;

    #[cfg(unix)]
    unsafe extern "C" {
        static mut environ: *mut *mut c_char;
    }

    static ENV: OnceLock<std::collections::HashMap<&'static str, &'static str>> = OnceLock::new();

    fn init() -> &'static HashMap<&'static str, &'static str> {
        ENV.get_or_init(|| unsafe {
            let mut map = HashMap::new();
            let mut ptr = environ;

            while !(*ptr).is_null() {
                let entry = CStr::from_ptr(*ptr);

                if let Ok(s) = entry.to_str()
                    && let Some((k, v)) = crate::split_kv(s)
                {
                    map.insert(k, v);
                }

                ptr = ptr.add(1);
            }

            map
        })
    }

    pub struct LibcBackend;

    impl EnvBackend for LibcBackend {
        fn get(key: &str) -> Option<&'static str> {
            init().get(key).copied()
        }
    }

    pub fn get(key: &str) -> Option<&'static str> {
        LibcBackend::get(key)
    }
}

#[cfg(feature = "std")]
pub use backend::get;

#[cfg(feature = "std")]
pub fn parse_env<T: FromStr>(key: &str) -> Option<T> {
    get(key)?.parse().ok()
}

#[cfg(feature = "std")]
pub mod dotenv {
    pub fn parse(buf: &'static str) -> impl Iterator<Item = (&'static str, &'static str)> {
        buf.lines().filter_map(|l| crate::split_kv(l))
    }
}
