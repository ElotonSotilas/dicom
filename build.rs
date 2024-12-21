use reqwest::blocking::get;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Error> {
    let url = "https://raw.githubusercontent.com/innolitics/dicom-standard/master/standard/attributes.json";
    let response = get(url)?;

    if response.status().is_success() {
        let dicom_tags: Value = response.json()?;
        let mut output = String::new();

        output.push_str("// AUTO-GENERATED FILE - DO NOT EDIT\n\n");
        output.push_str("pub mod dicom_groups {\n\n");

        let group_names: HashMap<&str, &str> = vec![
            ("0008", "File"),
            ("0010", "Patient"),
            ("0020", "Study"),
            ("0032", "FrameOfReference"),
            ("0040", "Scheduling"),
            ("0050", "Modality"),
            ("0070", "ViewPosition"),
        ]
        .into_iter()
        .collect();

        let mut grouped_elements: HashMap<String, Vec<(String, String, (u16, u16), String, String, bool)>> = HashMap::new();

        if let Some(elements) = dicom_tags.as_array() {
            for element_data in elements {
                if let Some(element_map) = element_data.as_object() {
                    let tag = element_map["tag"].as_str().unwrap_or("");
                    let name = element_map["name"].as_str().unwrap_or("").to_string();
                    let keyword = element_map["keyword"].as_str().unwrap_or("").to_string();
                    let vr = element_map["valueRepresentation"].as_str().unwrap_or("").to_string();
                    let vm = element_map["valueMultiplicity"].as_str().unwrap_or("").to_string();
                    let retired = element_map["retired"].as_str().unwrap_or("N") == "Y";
        
                    // Skip elements with empty required fields
                    if tag.is_empty() || name.is_empty() || keyword.is_empty() || vr.is_empty() || vm.is_empty() {
                        continue;
                    }
        
                    if let Some((group, element)) = parse_tag(tag) {
                        let group_name = format!("{:04X}", group);
                        if group_names.contains_key(group_name.as_str()) {
                            grouped_elements
                                .entry(group_name.clone())
                                .or_insert_with(Vec::new)
                                .push((keyword, name, (group, element), vr, vm, retired));
                        }
                    }
                }
            }
        }
        

        for (group_name, elements) in &grouped_elements {
            let group_mod = group_names[group_name.as_str()];
            output.push_str(&format!("    #[derive(Debug)]\n"));
            output.push_str(&format!("    pub struct {};\n", group_mod));
            
            // Derive Display for the group struct
            output.push_str(&format!("    impl std::fmt::Display for {} {{\n", group_mod));
            output.push_str(&format!("        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n"));
            output.push_str(&format!("            write!(f, \"{{:#?}}\", self)\n"));
            output.push_str("        }\n");
            output.push_str("    }\n\n");
            
            // Implement DicomTag for the group struct
            output.push_str(&format!("    impl crate::core::tag::DicomTag for {} {{\n", group_mod));
            output.push_str(&format!("        fn name(&self) -> String {{ \"{}\".to_string() }}\n", group_mod));

            let group_number = u16::from_str_radix(group_name, 16).unwrap();
            output.push_str(&format!("        fn tag(&self) -> (u16, u16) {{ (0x{:04X}, 0x0000) }}\n", group_number));
            output.push_str(&format!("        fn vr(&self) -> crate::core::tag::VisualRepresentation {{ crate::core::tag::VisualRepresentation::new(\"\") }}\n"));
            output.push_str(&format!("        fn group(&self) -> u16 {{ 0x{:04X} }}\n", group_number));
            output.push_str(&format!("        fn element(&self) -> Option<u16> {{ None }}\n"));
            output.push_str(&format!("        fn is_deprecated(&self) -> bool {{ false }}\n"));
            output.push_str(&format!("        fn multiplicity(&self) -> &str {{ \"\" }}\n"));
            output.push_str("    }\n\n");
        
            output.push_str(&format!("    impl {} {{\n", group_mod));
            for (keyword, _, _, _, _, _) in elements {
                output.push_str(&format!(
                    "        pub type {} = {}::{};\n",
                    keyword, group_mod.to_lowercase(), keyword
                ));
            }
            output.push_str("    }\n\n");
        
            output.push_str(&format!("    pub mod {} {{\n", group_mod.to_lowercase()));
            for (keyword, name, (group, element), vr, vm, retired) in elements {
                output.push_str(&format!("    #[derive(Debug)]\n"));
                output.push_str(&format!("        pub struct {};\n", keyword));
        
                // Deriving Display for the element struct
                output.push_str(&format!("        impl std::fmt::Display for {} {{\n", keyword));
                output.push_str(&format!("            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n"));
                output.push_str(&format!("                write!(f, \"{{:#?}}\", self)\n"));
                output.push_str("            }\n");
                output.push_str("        }\n\n");

                // Implement DicomTag for the child element
                output.push_str(&format!("        impl crate::core::tag::DicomTag for {} {{\n", keyword));
                output.push_str(&format!("            fn name(&self) -> String {{ \"{}\".to_string() }}\n", name));
                output.push_str(&format!("            fn tag(&self) -> (u16, u16) {{ (0x{:04X}, 0x{:04X}) }}\n", group, element));
                output.push_str(&format!("            fn vr(&self) -> crate::core::tag::VisualRepresentation {{ crate::core::tag::VisualRepresentation::new(\"{}\") }}\n", vr));
                output.push_str(&format!("            fn group(&self) -> u16 {{ 0x{:04X} }}\n", group));
                output.push_str(&format!(
                    "            fn element(&self) -> Option<u16> {{ Some(0x{:04X}) }}\n",
                    element
                ));
                output.push_str(&format!("            fn is_deprecated(&self) -> bool {{ {} }}\n", retired));
                output.push_str(&format!("            fn multiplicity(&self) -> &str {{ \"{}\" }}\n", vm));
                output.push_str("        }\n\n");
            }
            output.push_str("    }\n\n");
        }
        

        output.push_str("}\n");

        fs::write("src/core/generated.rs", output).expect("Unable to write file");

        let status = Command::new("rustfmt")
            .arg("src/core/generated.rs")
            .status()
            .expect("Failed to execute rustfmt");

        if !status.success() {
            eprintln!("Failed to format code with rustfmt");
        }

        println!("Code generation complete.");
        Ok(())
    } else {
        eprintln!("Failed to fetch DICOM attributes JSON");
        Ok(())
    }
}

fn parse_tag(tag: &str) -> Option<(u16, u16)> {
    let tag_parts: Vec<&str> = tag.trim_matches(|c| c == '(' || c == ')').split(',').collect();
    if tag_parts.len() == 2 {
        let group = u16::from_str_radix(tag_parts[0].trim(), 16).ok()?;
        let element = u16::from_str_radix(tag_parts[1].trim(), 16).ok()?;
        Some((group, element))
    } else {
        None
    }
}