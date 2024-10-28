pub mod constants {
    pub const TEST_DOMAIN: &str = "foo.com";
    pub const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";
    pub const TEST_BAZ_SUBDOMAIN: &str = "baz.foo.com";
    pub const READ_ERROR: &str = "Cannot read file!";
}

pub mod funcs {
    use super::constants::READ_ERROR;
    use std::fs;
    use std::path::{Path, PathBuf};
    use subscan::enums::content::Content;

    fn testdata_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("testing/testdata")
    }

    pub fn read_testdata(path: &str) -> Content {
        Content::String(fs::read_to_string(testdata_path().join(path)).expect(READ_ERROR))
    }
}
