use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use tempfile::{tempdir_in, TempDir};
use tokio::sync::OnceCell;

use super::utils::{get_random_port, stubs_path};

pub struct StubTemplateManager<'a> {
    pub temp: TempDir,
    pub stubs: PathBuf,
    pub templates: Vec<&'a str>,
    pub port: u16,
    pub init: OnceCell<()>,
}

impl<'a> From<(&'a str, Vec<&'a str>)> for StubTemplateManager<'a> {
    fn from(tuple: (&'a str, Vec<&'a str>)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl<'a> StubTemplateManager<'a> {
    pub fn new(stubs: &'a str, templates: Vec<&'a str>) -> Self {
        let stubs = stubs_path().join(stubs);
        let temp = tempdir_in(stubs.clone()).unwrap();
        let port = get_random_port();
        let init = OnceCell::new();

        Self {
            temp,
            stubs,
            templates,
            port,
            init,
        }
    }

    pub async fn port(&self) -> u16 {
        self.init().await;
        self.port
    }

    pub async fn temp(&self) -> &str {
        self.init().await;
        self.temp.path().to_str().unwrap()
    }

    async fn init(&self) {
        let init = || async {
            for entry in self.stubs.read_dir().unwrap().flatten() {
                if entry.path().is_file() {
                    let name = entry.file_name();
                    let target_path = self.temp.path().join(&name);

                    if self.templates.contains(&name.to_str().unwrap()) {
                        self.fill_port(entry.path(), target_path).await;
                    } else {
                        fs::copy(entry.path(), target_path).unwrap();
                    }
                }
            }
        };

        self.init.get_or_init(init).await;
    }

    async fn fill_port(&self, input: PathBuf, output: PathBuf) {
        let template = fs::read_to_string(input).unwrap();
        let filled_stub = template.replace("{{port}}", &self.port.to_string());
        let mut temp_stub = File::create(output).unwrap();

        temp_stub.write_all(filled_stub.as_bytes()).unwrap();
    }
}
