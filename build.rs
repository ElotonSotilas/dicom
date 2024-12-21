use std::fs;
use std::path::Path;

fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let json_path = project_root.join("dicom_tags.json");
    let output_path = project_root.join("src/core/generated.rs");

    println!("cargo:rerun-if-changed={}", json_path.display());

    // Check if dicom_tags.json exists, and create it with a template if missing
    if !json_path.exists() {
        let default_json_content = r#"{}"#;
        fs::write(&json_path, default_json_content)
            .expect("Failed to create default dicom_tags.json file");
        println!(
            "Created default dicom_tags.json at {}",
            json_path.display()
        );
    }

    // Read and parse the JSON file
    let json_content = fs::read_to_string(&json_path)
        .expect("Failed to read dicom_tags.json file");
    let dicom_tags: serde_json::Value =
        serde_json::from_str(&json_content).expect("Invalid JSON format");

    let mut output = String::new();
    output.push_str("// AUTO-GENERATED FILE - DO NOT EDIT\n\n");
    output.push_str("pub mod dicom_groups {\n\n");

    for (group_name, elements) in dicom_tags.as_object().unwrap() {
        output.push_str(&format!("    pub struct {};\n", group_name));
        output.push_str(&format!("    impl {} {{\n", group_name));

        for (element_name, element_data) in elements.as_object().unwrap() {
            let tag = element_data["tag"].as_str().unwrap();
            let vr = element_data["vr"].as_str().unwrap();
            let deprecated = element_data["deprecated"].as_bool().unwrap_or(false);

            output.push_str(&format!(
                "        pub struct {};\n",
                element_name
            ));

            output.push_str(&format!(
                "        impl super::super::DicomTag for {}::{} {{\n",
                group_name, element_name
            ));

            // Generate the `name()` method
            output.push_str(&format!(
                "            fn name(&self) -> String {{ \"{}\".to_string() }}\n",
                element_name
            ));

            let tag_parts: Vec<&str> = tag.split(',').collect();
            output.push_str(&format!(
                "            fn tag(&self) -> (u16, u16) {{ ({}, {}) }}\n",
                tag_parts[0].trim(),
                tag_parts[1].trim()
            ));

            output.push_str(&format!(
                "            fn vr(&mut self) -> &mut VisualRepresentation {{ VisualRepresentation::new(\"{}\") }}\n",
                vr
            ));

            output.push_str(&format!(
                "            fn is_deprecated(&self) -> bool {{ {} }}\n",
                deprecated
            ));

            output.push_str("        }\n");
        }

        output.push_str("    }\n");
    }

    output.push_str("}\n");

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directories for generated.rs");
        }
    }

    fs::write(&output_path, output).expect("Failed to write generated DICOM tags to file");
    println!("Generated Rust code written to {}", output_path.display());
}
