use serde::Deserialize;

//use crate::error::Handler;

#[derive(Deserialize, Clone)]
pub struct Metadata {
    pub title: Option<String>,
    pub default_argument: Option<String>
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub metadata: Option<Metadata>
}

#[derive(Deserialize)]
pub struct Cargo {
    pub package: Package
}

pub fn get() -> Package {
    let cargo_string = String::from_utf8_lossy( include_bytes!("../Cargo.toml") );
    let cargo: Cargo = toml::from_str(&cargo_string).unwrap(); // HANDLE

    cargo.package
}

impl Package {
    pub fn get_metadata( &self ) -> (String, String) {
        let mut title = self.name.clone();
        let mut default_argument = String::from("args");

        if self.metadata.is_some() {
            title = match self.metadata.clone().unwrap().title {
                Some(title) => title,
                None => title
            };

            default_argument = match self.metadata.clone().unwrap().default_argument {
                Some(default_argument) => default_argument,
                None => default_argument
            };
        }

        (title, default_argument)
    }
}
