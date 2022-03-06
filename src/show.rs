use colored::Colorize;
use std::{io::{stdout, Write}, process::exit};

use crate::{info, args, error::Handler};

const FORMAT_CHAR: char = '`';
const ARROW_CHAR: &'static str = "❯";

#[derive(PartialEq)]
pub enum Color {
    Green,
    Red,
    Yellow,
    Purple,
    Cyan
}

fn get_highlighted_text( formated_text: &str, color: Color ) -> String {
    let mut text_string = String::from(formated_text);
 
    if text_string.matches(FORMAT_CHAR).count() % 2 != 0 {
        panic!("The number of format chars in the string is not even");
    }

    while text_string.matches(FORMAT_CHAR).count() > 0 {
        let words: Vec<&str> = text_string.splitn(3, FORMAT_CHAR).collect();

        let highlighted_word = match color {
            Color::Green  => Colorize::green( words[1] ),
            Color::Red    => Colorize::red( words[1] ),
            Color::Yellow => Colorize::yellow( words[1] ),
            Color::Purple => Colorize::purple( words[1] ),
            Color::Cyan   => Colorize::cyan( words[1] )
        };

        text_string = format!( "{}{}{}", words[0].white(), highlighted_word, words[2].white() );
    }

    text_string
}

fn show_message( message: &str, color: Color, print_newline: bool ) {
    let message = format!("{}", message);
    let formatted_message = get_highlighted_text(&message, color);

    print!("{}", formatted_message);

    stdout().flush().handle(); // Flush to print immedialty

    if print_newline {
        println!();
    }
}

pub fn error( message: &str ) -> ! {
    let pointer = format!("{} {}", "Error", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), Color::Red, true );
    exit(1)
}

pub fn warning( message: &str ) {
    let pointer = format!("{} {}", "Warning", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), Color::Yellow, true );
}

pub fn information( label: &str, message: &str, color: Color ) {
    let pointer = format!("{} {}", label, ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), color, true );
}

pub fn data( message: &str, color: Color ) {
    let pointer = format!("\t{}", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), color, true );
}

pub fn title( title: &str, color: Color ) {
    show_message( &format!("`{}`", title.bold()), color, true );
}

pub fn help( options: Vec<String> ) {
    let info = info::get();

    let (title, default_argument) = info.get_metadata();

    let description = match info.description {
        Some(description) => format!(", {}", description),
        None => String::from("")
    };

    let name_message = format!( "`{title}` {version}{description}",
        title = title.bold(),
        version = info.version,
        description = description,
    );

    let usage_message = format!( "`{title}` {command} {options} {folder}\n",
        title = "Usage:".bold(),
        command = info.name,
        options = "[Options]...".purple(),
        folder = format!("<{}>", default_argument).blue(),
    );

    let options_message = format!( "`{}`", "Options:".bold() );

    show_message(&name_message, Color::Yellow, true);
    show_message(&usage_message, Color::Red, true);
    show_message(&options_message, Color::Purple, true);

    for option in &args::Option::from(&options) {
        let tabs = match option.long.chars().count() {
            0..=7 => "\t\t\t", 
            8..=16 => "\t\t",
            _ => "\t",
        };

        println!( "  {short}, {long}{tabs}{description}",
            short = format!( "-{}", option.short ).bold(),
            long = format!( "--{}", option.long ).bold(),
            tabs = tabs,
            description = option.description,
        );
    }

    exit(0)
}

pub fn version() {
    let info = info::get();

    let title = match info.metadata {
        Some(metadata) => match metadata.title {
            Some(title) => title,
            None => info.name.clone()
        },
        None => info.name.clone()
    };

    let version_message = format!("`{pointer} {title}` version `{version}`", 
        pointer = ARROW_CHAR.bold(),
        title = title.bold(),
        version = info.version
    );

    show_message(&version_message, Color::Green, true);

    exit(0)
}
