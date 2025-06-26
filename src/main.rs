use std::{
    fs::File,
    io::{BufWriter, IsTerminal},
    process::exit,
};

use oxidendron::Huffman;

#[repr(i32)]
enum ExitCode {
    MissingMode = 1,
    UnknownMode = 2,
    MissingArgument = 3,
    FailedToOpenInput = 4,
    FailedToOpenOutput = 5,
    NoOutput = 6,
}

mod consts;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("You have to supply a mode");
        exit(ExitCode::MissingMode as i32);
    }
    let mode = &args[1];
    match mode.as_str() {
        "encode" => encode(args),
        "decode" => decode(args),
        "version" => version(),
        "help" => help(args),
        _ => {
            eprintln!("Unknown mode, please use help for more information");
            exit(ExitCode::UnknownMode as i32);
        }
    }
}
fn help(args: Vec<String>) {
    use consts::{DECODE_MSG, ENCODE_MSG, HELP_MSG};
    let message = match args.get(2).map(String::as_str) {
        Some("decode") => DECODE_MSG,
        Some("encode") => ENCODE_MSG,
        _ => HELP_MSG,
    };
    println!("{message}");
}
fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
fn get_output(args: &mut Vec<String>) -> Option<String> {
    if let Some((output_index, _)) = args
        .iter()
        .enumerate()
        .find(|(_, arg)| arg.as_str() == "--output" || arg.as_str() == "-o")
    {
        args.remove(output_index);
        Some(args.remove(output_index))
    } else {
        None
    }
}
fn encode(mut args: Vec<String>) {
    let output = get_output(&mut args);
    if output.is_none() && std::io::stdout().is_terminal() {
        eprintln!(
            "You are currently trying to use this command without piping it somewhere or specifying an output source. Operation cancled"
        );
        exit(ExitCode::NoOutput as i32);
    }
    if args.len() < 3 {
        eprintln!("You have to supply an input file");
        exit(ExitCode::MissingArgument as i32)
    }
    let data = std::fs::read(&args[2]).unwrap_or_else(|e| {
        eprintln!("Failed to open input file: {e:?}");
        exit(ExitCode::FailedToOpenInput as i32);
    });
    if let Some(output) = output {
        let output_file = File::create(output).unwrap_or_else(|e| {
            eprintln!("Failed to open output file: {e:?}");
            exit(ExitCode::FailedToOpenOutput as i32);
        });
        Huffman::encode(&data, &mut BufWriter::new(output_file));
    } else {
        Huffman::encode(&data, &mut BufWriter::new(std::io::stdout()));
    }
}
fn decode(mut args: Vec<String>) {
    let output = get_output(&mut args);
    if output.is_none() && std::io::stdout().is_terminal() {
        eprintln!(
            "You are currently trying to use this command without piping it somewhere or specifying an output source. Operation cancled"
        );
        exit(ExitCode::NoOutput as i32);
    }
    if args.len() < 3 {
        eprintln!("You have to supply an input file");
        exit(ExitCode::MissingArgument as i32)
    }
    let data = std::fs::read(&args[2]).unwrap_or_else(|e| {
        eprintln!("Failed to open input file: {e:?}");
        exit(ExitCode::FailedToOpenInput as i32);
    });
    if let Some(output) = output {
        let output_file = File::create(output).unwrap_or_else(|e| {
            eprintln!("Failed to open output file: {e:?}");
            exit(ExitCode::FailedToOpenOutput as i32);
        });
        Huffman::decode(&data, &mut BufWriter::new(output_file));
    } else {
        Huffman::decode(&data, &mut BufWriter::new(std::io::stdout()));
    }
}
