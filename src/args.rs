use std::env;

use serde::{Serialize, de::DeserializeOwned};

use crate::error::{self, Handler};

pub struct Option {
    pub long: String,
    pub short: char,
    pub description: String
}

impl Option {
    pub fn new( field_name: String, field_names: &Vec<String> ) -> Self {
        let char = get_char_from_field_name(field_name.clone(), field_names);
        let description = match field_name.clone().as_str() {
            "help" => String::from("Show this help message"),
            "version" => String::from("Show application version"),
            _ => String::from("description (TODO)")
        };

        Self {
            long: field_name,
            short: char,
            description
        }
    }

    pub fn from( field_names: &Vec<String> ) -> Vec<Self> {
        let mut options = Vec::new();

        for field_name in field_names {
            options.push( Self::new(field_name.clone(), field_names) )
        }

        options
    }
}

fn get_struct_fields( serialized_struct: String ) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_str(&serialized_struct).unwrap();
    let keys: Vec<String> = value.as_object().unwrap().keys().map( |k| k.clone() ).collect();
 
    keys
}

fn get_options_from_struct( serialized_struct: String ) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_str(&serialized_struct).unwrap();
    let mut options: Vec<String> = Vec::new();

    options.push( String::from("help") );
    options.push( String::from("version") );

    for key in value.as_object().unwrap().keys() {
        options.push( key.clone().replace("_", "-") );
    }

    options
}

pub fn get_char_from_field_name( field_name: String, field_names: &Vec<String> ) -> char {
    if field_name == "help" {
        return 'h';
    } else if field_name == "version" {
        return 'V';
    }

    let mut names_with_same_start = Vec::new();

    for name in field_names {
        if field_name.chars().nth(0).unwrap() == name.chars().nth(0).unwrap() {
            names_with_same_start.push( name.clone() )
        }
    }

    if names_with_same_start.len() == 1 {
        return field_name.chars().nth(0).unwrap();

    } else if names_with_same_start.len() == 2 {
        if field_name == names_with_same_start[0] {
            return field_name.chars().nth(0).unwrap();
        } else {
            return field_name.chars().nth(0).unwrap().to_ascii_uppercase();
        }
    } 

    error::raise(error::Kind::OptionsTooSimilar, field_name.chars().nth(0).unwrap())
}

fn get_option_from_char( char: char, serialized_struct: String ) -> String {
    let keys = get_struct_fields(serialized_struct);

    if char == 'h' {
        return  String::from("help");
    } else if char == 'V' {
        return String::from("version");
    }

    let mut first_char_matches = Vec::new();

    for key in keys {
        if key.chars().nth(0).unwrap() == char.to_ascii_lowercase() {
            first_char_matches.push(key.clone())
        }
    }

    if !first_char_matches.is_empty() {
        if first_char_matches.len() == 1 {
            return first_char_matches[0].clone();

        } else if first_char_matches.len() == 2 {
            if char.is_lowercase() {
                return first_char_matches[0].clone();
            } else {
                return first_char_matches[1].clone();
            }
        }

        error::raise(error::Kind::OptionsTooSimilar, char)
        
    } else {
        error::raise(error::Kind::InvalidOption, char)
    }
}

fn get_serialized_arguments( serialized_struct: String, cargo_bytes: &[u8] ) -> String {
    let arguments: Vec<String> = env::args().collect();

    let mut serialized_arguments = String::new();
    let mut is_first_argument = true;

    if arguments.len() > 1 {
        for (index, argument) in arguments[1..].into_iter().enumerate() {
            if argument.starts_with("-") {
                let argument_names = match argument.starts_with("--") {
                    true => vec![ argument[2..].replace("-", "_") ],
                    false => argument.chars().skip(1).map( |c| get_option_from_char(c, serialized_struct.clone()) ).collect()
                };

                for argument_name in argument_names {
                    if argument_name == "help" {
                        crate::show::help( get_options_from_struct(serialized_struct.clone()), cargo_bytes );
                        
                    } else if argument_name == "version" {
                        crate::show::version(cargo_bytes);
    
                    } else {
                        if !get_struct_fields(serialized_struct.clone()).contains(&argument_name) {
                            error::raise(error::Kind::InvalidOption, argument_name)
                        }

                        let mut key_string = format!(", \"{}\":", argument_name);

                        if is_first_argument {
                            is_first_argument = false;
                            key_string = key_string[2..].to_string();
                        }
                            
                        serialized_arguments.push_str(&key_string);

                        if index + 2 < arguments.len() && arguments[index + 2].starts_with("-") {
                            serialized_arguments.push_str(" true");

                        } else if index + 2 == arguments.len() {
                            serialized_arguments.push_str(" true");
                        }
                    }
                }
                
            } else {
                let mut value_string = String::new();

                let parsed_argument = match argument.chars().all( |c| c.is_numeric() ) {
                    true => argument.clone(),
                    false => format!("\"{}\"", argument)
                };

                if serialized_arguments.chars().last().is_none() {
                    // Pass unamed arg to the first field of the strict
                    let key_string = format!( "\"{}\":", get_struct_fields(serialized_struct.clone())[0] );
                    serialized_arguments.push_str(&key_string);
                }    

                if serialized_arguments.chars().last().unwrap() != ',' && index + 2 < arguments.len() && !arguments[index + 2].starts_with("-") {
                    value_string.push_str( &format!(" [{},", parsed_argument) );
                    
                } else {
                    if serialized_arguments.chars().last().unwrap() == ':' {
                        value_string.push_str( &format!(" {}", parsed_argument) );

                    } else {
                        if index + 2 < arguments.len() && !arguments[index + 2].starts_with("-") {
                            value_string.push_str( &format!(" {},", parsed_argument) );
                        } else {
                            value_string.push_str( &format!(" {}]", parsed_argument) );
                        }
                    }
                }

                serialized_arguments.push_str(&value_string);
            }
        }
    }

    serialized_arguments = format!( "{{ {} }}", serialized_arguments );

    serialized_arguments
}

pub fn parse_into<T>( args_struct: &mut T, cargo_bytes: &[u8] )
where T: Serialize, T: DeserializeOwned, T: std::fmt::Debug {
    let serialized_struct = serde_json::to_string(&args_struct).unwrap();
    let serialized_arguments = get_serialized_arguments(serialized_struct, cargo_bytes);
    let deserialized_struct: T = serde_json::from_str(&serialized_arguments).handle();

    *args_struct = deserialized_struct;
}
