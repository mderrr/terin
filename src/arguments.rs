use crate::error;
use crate::display;
use crate::config::Config;

pub struct Option {
    pub long: &'static str,
    pub short: char,
    pub description: &'static str
}

pub const HELP_OPTION: Option = Option { long: "help", short: 'h', description: "Show this help message" };
pub const VERSION_OPTION: Option = Option { long: "version", short: 'V', description: "Show program version" };
pub const QUIET_OPTION: Option = Option { long: "quiet", short: 'q', description: "Enable quiet mode" };
pub const POSSIBILITIES_OPTION: Option = Option { long: "possibilities-for", short: 'p', description: "Show possible enrollments for student with this index" };

pub const OPTIONS: [Option; 4] = [
    HELP_OPTION,
    VERSION_OPTION,
    QUIET_OPTION,
    POSSIBILITIES_OPTION
];

enum PreviousOption {
    PossibilitiesFor,
    None
}

pub fn read( arguments: Vec<String>, config: &mut Config ) {
    let mut previous_option = PreviousOption::None;

    if arguments.len() > 1 {
        for argument in &arguments[1..] {
            if argument.starts_with("--") {
                match &argument[2..] {
                    x if x == HELP_OPTION.long => display::help(),
                    x if x == VERSION_OPTION.long => display::version(),
                    x if x == QUIET_OPTION.long => config.be_quiet = true,
                    x if x == POSSIBILITIES_OPTION.long => previous_option = PreviousOption::PossibilitiesFor,

                    _ => error::raise( error::Kind::InvalidOption, argument ),
                }

            } else if argument.starts_with("-") {
                for char in argument.chars().skip(1) {
                    match char {
                        x if x == HELP_OPTION.short => display::help(),
                        x if x == VERSION_OPTION.short => display::version(),
                        x if x == QUIET_OPTION.short => config.be_quiet = true,
                        x if x == POSSIBILITIES_OPTION.short => previous_option = PreviousOption::PossibilitiesFor,

                        _ => error::raise( error::Kind::InvalidOption, char ),
                    }
                }
                
            } else {
                match previous_option {
                    PreviousOption::PossibilitiesFor => config.possibilities_for = argument.parse::<usize>().unwrap(),
                    PreviousOption::None => ()
                }

                previous_option = PreviousOption::None;
            }
        }
    }
}
