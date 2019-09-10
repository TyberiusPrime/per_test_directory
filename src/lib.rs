pub use per_test_directory_macros::per_test_dir;

use std::env::set_current_dir;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;

// used by the proc macro [per_test_dir]
//#[doc(hidden)]
pub struct PerTestDirectoryFixture {
    path: String,
    pub passed: bool,
}

impl PerTestDirectoryFixture {
    // used by the proc macro [per_test_dir]
    #[allow(dead_code)]
    pub fn new(path: String) -> PerTestDirectoryFixture {
        let fp = format!("test_runs/{}", path);
        let p = Path::new(&fp);
        create_dir_all(p).unwrap();
        set_current_dir(p).unwrap();
        PerTestDirectoryFixture {
            path,
            passed: false,
        }
    }
}

impl Drop for PerTestDirectoryFixture {
    fn drop(&mut self) {
        if self.passed {
            set_current_dir(Path::new("..")).unwrap();
            remove_dir_all(&self.path).ok();
            set_current_dir(Path::new("..")).unwrap();
        } else {
            set_current_dir(Path::new("../..")).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;
    use std::path::Path;
    #[test]
    fn test_cleanup() {
        {
            #[per_test_dir]
            fn inner(do_panic: bool) {
                if do_panic {
                    panic!("shu")
                }
            }
            let result = catch_unwind(|| inner(true));
            assert!(result.is_err());
            assert!(Path::new("test_runs").exists());
            assert!(Path::new("test_runs/tests.inner").exists());
            let result = catch_unwind(|| inner(false));
            assert!(result.is_ok());
            assert!(!Path::new("test_runs/tests.inner").exists());
        }
    }
}
