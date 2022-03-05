use std::{io::{stdout, Write}, process::exit};

use colored::Colorize;

use crate::info;

const FORMAT_CHAR: char = '`';
const ARROW_CHAR: &'static str = "❯";

#[derive(PartialEq)]
pub enum Color {
    Green,
    Red,
    Yellow,
    Purple
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
            Color::Purple => Colorize::purple( words[1] )
        };

        text_string = format!( "{}{}{}", words[0].white(), highlighted_word, words[2].white() );
    }

    text_string
}

fn show_message( message: &str, message_color: Color, print_newline: bool ) {
    let message = format!("{}", message);
    let formatted_message = get_highlighted_text(&message, message_color);

    print!("{}", formatted_message);

    stdout().flush().unwrap(); //HANDLE // Flush to print immedialty

    if print_newline {
        println!();
    }
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

    println!("AAA {:?}", options);

    // for flag in arguments::OPTIONS {
    //     let tabs = match flag.long.chars().count() {
    //         0..=8 => "\t\t\t", 
    //         9..=16 => "\t\t",
    //         _ => "\t",
    //     };

    //     println!( "  {short}, {long}{tabs}{description}",
    //         short = format!("-{}", flag.short).bold(),
    //         long = format!("--{}", flag.long).bold(),
    //         tabs = tabs,
    //         description = flag.description,
    //     );
    // }

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
