mod asm;
mod disassemble;
use disassemble::parse_rom;
use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("disassembler")
        .version("0.1.0")
        .author("PThorpe92")
        .about("Disassembles a CHIP-8 ROM into Hex or Assembly/Opcodes")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .help("Sets the input file ROM to disassemble")
                .required(true)
        ).arg(
            Arg::new("filetype")
                 .long("filetype")
                 .short('f')
                 .help("Specify either hex or bin")
                 .required(false) 
        ).get_matches();
    let file: String = matches.get_one::<String>("input").unwrap().to_owned();
    let path = PathBuf::from(file.as_str());
    let filetype: String = matches.get_one::<String>("filetype").unwrap().to_owned();
    match filetype.as_str() {
        "bin" => {
            match parse_rom(&path, false) {
                Ok(_) => {
                   println!("Successfully disassembled ROM");
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        },
        "hex" => {
            match parse_rom(&path, true) {
                Ok(_) => {
                    println!("Successfully disassembled ROM");
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        },
        _ => {
            println!("Invalid file type");
        },
    }
    let emulatormatches = Command::new("emulator")
                          .version("0.1.0")
                          .author("PThorpe92")
                          .about("Emulates and runs a CHIP-8 ROM")
                          .arg(
                           Arg::new("input_rom")
                                  .long("input")
                                  .short('i')
                                  .help("Sets the input file ROM to emulate")
                                  .required(true)
                          ).get_matches();
       let emulatorfile: String = emulatormatches.get_one::<String>("input").unwrap().to_owned();
         let emu_path = PathBuf::from(emulatorfile.as_str());
            match parse_rom(&emulatorpath, false) {
                    Ok(_) => {
                    run_emulator(emu_path);
                    println!("Successfully emulated ROM");
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    },
                }
    
}
