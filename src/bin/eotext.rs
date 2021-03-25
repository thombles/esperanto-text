/// Utility to transliterate Esperanto

use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut text = String::new();
    if args.len() < 3 {
        invalid_input(&args);
    } else if args.len() == 3 {
        io::stdin().read_to_string(&mut text)
            .expect("Could not read from stdin");
    } else {
        text = args[3..].join(" ");
    }

    let output = match (args[1].as_ref(), args[2].as_ref()) {
        ("u", "x") => esperanto_text::utf8_to_x_system(&text),
        ("x", "u") => esperanto_text::x_system_to_utf8(&text),
        ("u", "h") => esperanto_text::utf8_to_h_system(&text),
        ("h", "u") => esperanto_text::h_system_to_utf8(&text),
        ("x", "h") => {
            let utf8 = esperanto_text::x_system_to_utf8(&text);
            esperanto_text::utf8_to_h_system(&utf8)
        },
        ("h", "x") => {
            let utf8 = esperanto_text::h_system_to_utf8(&text);
            esperanto_text::utf8_to_x_system(&utf8)
        }
        ("h", "h") | ("u", "u") | ("x", "x") => text.clone(),
        _ => invalid_input(&args),
    };
    println!("{}", output);
}

fn invalid_input(args: &Vec<String>) -> ! {
    println!("Usage: {} <from> <to> [input text]", args[0]);
    println!("where `from` and `to` are one of the following letters:");
    println!("    u   UTF-8 input (with diacritics)");
    println!("    x   x-system input");
    println!("    h   h-system input");
    println!("If no input text is specified, it is read from standard input.");
    println!("Example: {} x u \"sxangxo\"", args[0]);
    std::process::exit(1);
}
