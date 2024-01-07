use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn main() {
    let output_file = File::create("Context.md").expect("Failed to create Context.md");
    let mut context = BufWriter::new(output_file);

    // Process the "src" directory
    process_directory(&mut context, Path::new("src")).expect("Failed to process src directory");

    // Process the "tests" directory, if it exists
    if Path::new("tests").exists() {
        process_directory(&mut context, Path::new("tests")).expect("Failed to process tests directory");
    }

    drop(context);

    // Run "cargo check" and append its output to Context.md
    std::process::Command::new("sh")
        .arg("-c")
        .arg("cargo check 2>&1 | tee -a Context.md")
        .output()
        .expect("Failed to run cargo check");
}

fn process_directory(context: &mut BufWriter<File>, path: &Path) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // If the entry is a directory, recursively process it
            process_directory(context, &path)?;
        } else {
            // Process files as before
            let file_name = entry.file_name().into_string().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid filename in directory {:?}", path),
                )
            })?;

            let content = fs::read_to_string(&path)?;
            let formatted_content = format!("## {}\n```rust\n{}\n```\n", file_name, content);

            context.write_all(formatted_content.as_bytes())?;
        }
    }

    Ok(())
}
