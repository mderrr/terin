use colored::Colorize;
use std::io::{stdout, prelude::*};

use crate::info;
use crate::arguments;
use crate::error::{self, Handler};

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

fn get_highlighted_text( formated_text: &str, message_color: Color ) -> String {
    let mut text_string = String::from(formated_text);
 
    if text_string.matches(FORMAT_CHAR).count() % 2 != 0 {
        panic!("The number of format chars in the string is not even");
    }

    while text_string.matches(FORMAT_CHAR).count() > 0 {
        let words: Vec<&str> = text_string.splitn(3, FORMAT_CHAR).collect();

        let highlighted_word = match message_color {
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

fn show_message( message: &str, message_color: Color, print_newline: bool ) {
    let message = format!("{}", message);
    let formatted_message = get_highlighted_text(&message, message_color);

    print!("{}", formatted_message);

    stdout().flush().handle(); // Flush to print immedialty

    if print_newline {
        println!();
    }
}

pub fn information( label: &str, message: &str, message_color: Color ) {
    let pointer = format!("{} {}", label, ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), message_color, true );
}

pub fn data( message: &str, message_color: Color ) {
    let pointer = format!("\t{}", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), message_color, true );
}

pub fn title( title: &str, message_color: Color ) {
    show_message( &format!("`{}`", title.bold()), message_color, true );
}

pub fn alert( message: &str ) {
    let pointer = format!("{} {}", "Error", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), Color::Red, true );
}

pub fn warning( message: &str ) {
    let pointer = format!("{} {}", "Warning", ARROW_CHAR);

    show_message( &format!("`{}` {}", pointer.bold(), message), Color::Yellow, true );
}

pub fn help() {
    let info = info::get();

    let title = match info.metadata {
        Some(metadata) => metadata.title,
        None => info.name.clone()
    };

    let description = match info.description {
        Some(description) => description,
        None => String::from("")
    };

    let name_message = format!( "`{title}` {version}, {description}",
        title = title.bold(),
        version = info.version,
        description = description,
    );

    let usage_message = format!( "`{title}` {command} {options} {folder}\n",
        title = "Usage:".bold(),
        command = info.name,
        options = "[Options]...".purple(),
        folder = "<xlsx file>".blue(),
    );

    let options_message = format!( "`{}`", "Options:".bold() );

    show_message(&name_message, Color::Yellow, true);
    show_message(&usage_message, Color::Red, true);
    show_message(&options_message, Color::Purple, true);

    for flag in arguments::OPTIONS {
        let tabs = match flag.long.chars().count() {
            0..=8 => "\t\t\t", 
            9..=16 => "\t\t",
            _ => "\t",
        };

        println!( "  {short}, {long}{tabs}{description}",
            short = format!("-{}", flag.short).bold(),
            long = format!("--{}", flag.long).bold(),
            tabs = tabs,
            description = flag.description,
        );
    }

    error::exit_with_success()
}

pub fn version() {
    let info = info::get();

    let title = match info.metadata {
        Some(metadata) => metadata.title,
        None => info.name.clone()
    };

    let version_message = format!("`{pointer} {title}` version `{version}`", 
        pointer = ARROW_CHAR.bold(),
        title = title.bold(),
        version = info.version
    );

    show_message(&version_message, Color::Green, true);

    error::exit_with_success()
}
