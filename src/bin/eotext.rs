/// Utility to transliterate Esperanto

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        invalid_input(&args);
    }

    let output = match (args[1].as_ref(), args[2].as_ref()) {
        ("u", "x") => esperanto_text::utf8_to_x_system(&args[3]),
        ("x", "u") => esperanto_text::x_system_to_utf8(&args[3]),
        ("u", "h") => esperanto_text::utf8_to_h_system(&args[3]),
        ("h", "u") => esperanto_text::h_system_to_utf8(&args[3]),
        ("h", "h") | ("u", "u") | ("x", "x") => args[3].clone(),
        _ => invalid_input(&args),
    };
    println!("{}", output);
}

fn invalid_input(args: &Vec<String>) -> ! {
    println!("Usage: {} <from> <to> \"<input text>\"", args[0]);
    println!("where `from` and `to` are one of the following letters:");
    println!("    u   UTF-8 input (with diacritics)");
    println!("    x   x-system input");
    println!("    h   h-system input");
    println!("Example: {} x u \"sxangxo\"", args[0]);
    std::process::exit(1);
}