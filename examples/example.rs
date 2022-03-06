use terin::{args, show};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Args {
    index: u8,
    text: Option<String>,
    count: Option<u8>
}

impl Args {
    fn new() -> Self {
        Self {
            index: 1,
            text: Some("test".to_string()),
            count: None
        }
    }
}

fn main() {
    let mut args = Args::new();

    args::parse_into(&mut args);
    show::information("Parsed args", &format!("{:?}", args), show::Color::Red) ;
}
