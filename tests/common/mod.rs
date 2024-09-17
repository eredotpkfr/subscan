pub mod constants {
    #![allow(dead_code)]
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_BAZ_SUBDOMAIN: &str = "baz.foo.com";
    pub const TEST_MODULE_NAME: &str = "foo-module";
    pub const TEST_URL: &str = "http://foo.com";
}

pub mod funcs {
    #![allow(dead_code)]
    use std::fs;
    use std::path::{Path, PathBuf};

    const READ_ERROR: &str = "Cannot read file!";

    fn testdata_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("testing/testdata")
    }

    pub fn read_testdata(path: &str) -> String {
        fs::read_to_string(testdata_path().join(path)).expect(READ_ERROR)
    }
}
