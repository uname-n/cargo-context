use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let output_file = File::create("Context.md").expect("Failed to create Context.md");
    let mut context = BufWriter::new(output_file);

    // Process the "src" directory
    process_directory(&mut context, "src").expect("Failed to process src directory");

    // Process the "tests" directory, if it exists
    if Path::new("tests").exists() {
        process_directory(&mut context, "tests").expect("Failed to process tests directory");
    }

    drop(context);

    // Run "cargo check" and append its output to Context.md
    std::process::Command::new("sh")
        .arg("-c")
        .arg("cargo check 2>&1 | tee -a Context.md")
        .output()
        .expect("Failed to run cargo check");
}

fn process_directory(context: &mut BufWriter<File>, dir: &str) -> std::io::Result<()> {
    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().into_string().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid filename in {} directory", dir),
            )
        })?;

        let content = std::fs::read_to_string(entry.path())?;
        let formatted_content = format!("## {}\n```rust\n{}\n```\n", file_name, content);

        context.write_all(formatted_content.as_bytes())?;
    }

    Ok(())
}
