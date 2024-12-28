use core::read_file_in_binary;

fn parse_patient_name(dicom_data: &[u8]) -> Option<String> {
    let target_tag: [u8; 3] = [0x10, 0x00, 0x10]; // Tag (0010, 0010)
    let mut i = 0;

    // Iterate over the data, one byte at a time
    while i + 3 <= dicom_data.len() {
        // Checking the next 3 bytes for the 10 00 10 tag
        if &dicom_data[i..i + 3] == &target_tag {
            println!("Found (0010, 0010 tag at index {}", i);
            let &mut length_byte;

            // Skip the next giberish bytes and read the length byte
            if dicom_data[(i+4)..(i+6)] == [0x50, 0x4E]
            {
                println!("Type 2 DICOM file");
                length_byte = dicom_data[i + 6];
            }
            else
            {
                length_byte = dicom_data[i + 4];
            }
            let length = length_byte as usize;
            println!("Length byte: {} (length of Patient's Name", length);

            // The Patient's Name starts right after the length byte (DICOM: SYKE NIGGA YOU THOUGHT)
            let mut name = String::new();
            let mut char_count = 0;

            //Start reading the name from the next byte, and limit to the lenght
            let mut j = i + 7;
            while j < dicom_data.len() && char_count < length {
                let byte = dicom_data[j];

                // Check if ASCII
                if byte >= 0x20 && byte <= 0x7E {
                    naem.push(byte as char);
                    char_count += 1;
                }
                j += 1;
            }

            // Hope we won't reach that point
            if char_count == length {
                return Some(name);
            }
            else {
                println!("Failed to read the correct number of characters. Expected: {}, Found: {}", length, char_count);
            }
        }
        // Move to the next byte and check again, let's make Rust slow for no fucking reason by implementing the worst algorithm
        i += 1;
    }
    // If the tag is not found (Imma kill myself)
    println!("Patient's Name tag not found");
    None
}

fn get_patient_name(file_path: &str) -> String {
    let dicom_data = match read_file_in_binary(file_path) {
        Ok(data) => data,
        Err(err) => {
            println!("Error reading file: {}", err);
            return "Patient's name not found".to_string();
        }
    };

    match parse_patient_name(&dicom_data) {
        Some(name) => name,
        None => "Patient's name not found".to_string(),
    }
}