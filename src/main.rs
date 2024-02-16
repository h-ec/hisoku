mod reader;
mod parser;
pub mod utils;

use std::{self, env};

use crate::reader::read_file;

fn main() {
    // We get the args as a vector
    let args: Vec<String> = env::args().collect();

    // If args length is less than 2 (Which means no arg-command used in hisoku do nothing)
    if args.len() < 2 {
        panic!(
            "> Hisoku: No arguments were entered, Are you sure {} argument(s) is enough?",
            args.len() - 1
        );
    }
    
    // We get the fsec command usage and we remove dashes from it
    let fscu_binding: String = args[1].to_string();
    let fsec_command_usage: &str = &fscu_binding.as_str().replace("-", "");

    // We just match the fsec_command_usage to we can call a function for each command and it's code
    match fsec_command_usage {
        "run" => {
            // If there's no file path, PANIC!
            if args.len() < 3 {
                panic!(
                    "> Hisoku: No file path were entered, Are you sure you added the file path argument?",
                )
            }

            // We get the file content through the read_file function
            let result = read_file(args[2].as_str());
            match result {
                Ok(value) => println!("{}", parser::parse_content(value.to_string().as_str()
            )),
                // If there's an error, PANIC! (Probably the file doesn't exist or it is a directory)
                Err(_) => panic!(
                    "> Hisoku: No file path were entered, Are you sure {} is a file path?",
                    args[2]
                )
            }
        },
        
        // Otherwise if nothing that are listed in the command list in the top ^, Just tell him it's not a command and exit process.
        ucu => println!("> Hisoku: {} is not a supported argument usage nor found a command that matches {}.", ucu, ucu)
    }
}