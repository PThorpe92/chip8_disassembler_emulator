use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, self};
use hex::encode;
use crate::asm::return_opcodes;

fn parse_rom_to_hex(file: &Path) -> io::Result<Box<File>> {
    let mut rom = File::open(file)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;

    let hex_data = encode(&buffer);

    let path = Path::new("./rom.hex");
    let mut new_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    let hex: Vec<&str> = hex_data
        .as_bytes()
        .chunks(2)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();
    
    for byte in &hex {
    println!("{}", byte);
    new_file.write(format!("0x{}\n",byte).as_bytes())?;
    }
    let ret_file = Box::new(new_file);
    return Ok(ret_file);
}

 pub fn parse_rom(file: &Path, hex: bool) -> io::Result<()> {
    let path = Path::new("./rom.asm");
    let mut new_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    if !hex {
  let mut hex_file = parse_rom_to_hex(file)?;
  let mut buffer = Vec::new();
    hex_file.read_to_end(&mut buffer)?;
    println!("{}", String::from_utf8_lossy(&buffer));
        for byte in buffer {
            print!("{:x}", byte);
            let opcode = return_opcodes(byte);
            match new_file.write(format!("{}\n", opcode).as_bytes()) {
                Ok(_) => (),
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
    Ok(())
    } else {
        let mut hex_file = OpenOptions::new()
                            .write(false)
                            .read(true)
                            .open(file)?;
        let mut buffer = Vec::new();
        hex_file.read_to_end(&mut buffer)?;
        for byte in buffer {
            print!("{:x}", byte);
            let opcode = return_opcodes(byte);
            match new_file.write(format!("{}\n", opcode).as_bytes()) {
                Ok(_) => (),
                Err(e) => println!("Error writing to file: {}", e),
            }
        }
    Ok(())
    }
}
