use std::io::BufWriter;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut context = BufWriter::new(File::create("Context.md").unwrap());
    
    for file in std::fs::read_dir("src").unwrap() {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();

        let content = std::fs::read_to_string(file.path()).unwrap();
        let content = format!("## {}\n```rust\n{}\n```\n", file_name, content);
        
        context.write_all(content.as_bytes()).unwrap();
    }

    for file in std::fs::read_dir("tests").unwrap() {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();

        let content = std::fs::read_to_string(file.path()).unwrap();
        let content = format!("## {}\n```rust\n{}\n```\n", file_name, content);
        
        context.write_all(content.as_bytes()).unwrap();
    }
}
