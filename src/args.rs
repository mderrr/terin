use std::env;

use serde::{Serialize, de::DeserializeOwned};

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
    //let keys: Vec<String> = value.as_object().unwrap().keys().map( |k| k.clone() ).collect();

    options
}

fn get_char_from_option( option: String, options: Vec<String> ) -> char {
    if option == "help" {
        return 'h';
    } else if option == "version" {
        return 'V';
    }

    if !options.contains( &option.chars().nth(0).unwrap().to_string() ) {
        return option.chars().nth(0).unwrap();

    } else {
        if !options.contains( &option.chars().nth(0).unwrap().to_ascii_uppercase().to_string() ) {
            return option.chars().nth(0).unwrap().to_ascii_uppercase();

        } else {
            panic!("Too many")
        }
    }
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

        panic!("Too many")
        
    } else {
        panic!("Not found")
    }
}

fn get_serialized_arguments( serialized_struct: String ) -> String {
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
                        crate::show::help( get_options_from_struct(serialized_struct.clone()) );
                        
                    } else if argument_name == "version" {
                        crate::show::version();
    
                    } else {
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

pub fn parse_into<T>( config: &mut T )
where T: Serialize, T: DeserializeOwned, T: std::fmt::Debug {
    let arguments: Vec<String> = env::args().collect();

    println!("{:?}", arguments);

    let serialized_struct = serde_json::to_string(&config).unwrap();
    let serialized_arguments = get_serialized_arguments(serialized_struct);
    let deserialized_struct: T = serde_json::from_str(&serialized_arguments).unwrap();

    *config = deserialized_struct;
}
