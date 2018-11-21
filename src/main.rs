extern crate hex;

use std::env;
use std::fs;
use std::ffi::OsString;
use std::process;

fn main()
{
    let args: Vec<OsString> = env::args_os().collect();
    if args.len() != 3
    {
        println!("Usage: pp_table_converter input_powerplay_file.reg output_powerplay_file.bin");
        process::exit(0);
    }
    let windows_pp_table = fs::read_to_string(&args[1]).expect("Unable to read input powerplay registry file. Please check that the path is correct or the file is present. Exiting with errors.");
    let chars_to_trim: &[char] = &[' ', ',', '\\', '\n', '\r'];
    let hex_pp_table = &windows_pp_table[windows_pp_table.find(":").unwrap_or(windows_pp_table.len() - 1) + 1..].replace(chars_to_trim, "");
    let binary_pp_table = hex::decode(hex_pp_table).expect("Error parsing input powerplay registry file. Double check if the hexidecimal is correct. Exiting with errors.");
    fs::write(&args[2], binary_pp_table).expect("Unable to write to output binary Linux powerplay file. Exiting with errors.");
    println!("Wrote output Powerplay binary file at {:?}", args[2]);
}
