use terin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cosa {
    name: u8,
    jeje: String,
    count: Option<u8>,
    append_f: Option<String>,
    append_a: Option<String>
}

impl Cosa {
    fn new() -> Self {
        Self {
            name: 1,
            jeje: "".to_string(),
            count: None,
            append_f: None,
            append_a: None
        }
    }
}


fn main() {
    terin::print_a();

    let mut config = Cosa::new();

    terin::args::parse_into(&mut config);

    println!("AcTUALU {:?}", config);
}