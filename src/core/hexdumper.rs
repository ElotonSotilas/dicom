use std::fs::File;
use std::io::{self, Read};

pub fn read_file_in_binary(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn hex_and_ascii(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08X}: ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }

        for _ in chunk.len()..16 {
            print!("    ");
        }

        print!(" | ");
        for byte in chunk {
            let c = if byte.is_ascii_graphic() || *byte == b' '{
                *byte as char;
                    }
                    else{
                        '.'
                    };
            println!("{}", c);
        }
        println!();
    } 
}

pub fn dicom_hexdump(file_path: &str)
{
    let bin_data = read_file_in_binary(file_path).expect("Error reading from the DICOM file");
    hex_and_ascii(&bin_data);
}