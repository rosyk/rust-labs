use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::env;

fn create_text_file(filename: &str) -> io::Result<()> {
    File::create(filename)?;
    Ok(())
}

fn create_directory(dirname: &str) -> io::Result<()> {
    fs::create_dir(dirname)?;
    Ok(())
}

fn copy_file(source: &str, destination: &str) -> io::Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}

fn copy_directory_recursive(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if !source_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a directory",
        ));
    }

    fs::create_dir_all(destination_path)?;

    for entry in fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let destination_path_buf = destination_path.join(file_name);
        let destination_path_str = destination_path_buf.to_str().unwrap();

        if path.is_dir() {
            copy_directory_recursive(path.to_str().unwrap(), destination_path_str)?;
        } else {
            fs::copy(path, destination_path_str)?;
        }
    }

    Ok(())
}

fn search_file(directory: &str, filename: &str) -> io::Result<Option<PathBuf>> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().map(|name| name.to_string_lossy().to_string()) == Some(filename.to_string()) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn delete_file(filename: &str) -> io::Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}

fn delete_directory(dirname: &str) -> io::Result<()> {
    fs::remove_dir_all(dirname)?;
    Ok(())
}

fn delete_directories(dirnames: &[&str]) -> io::Result<()> {
    for &dirname in dirnames {
        fs::remove_dir_all(dirname)?;
    }
    Ok(())
}

fn list_directory_contents(dirname: &str) -> io::Result<()> {
    println!("Contents of directory '{}':", dirname);
    for entry in fs::read_dir(dirname)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

fn view_file_properties(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let metadata = fs::metadata(path)?;

    println!("File properties for '{}':", filename);
    println!("  Type: {:?}", metadata.file_type());
    println!("  Size: {} bytes", metadata.len());
    println!("  Permissions: {:?}", metadata.permissions());
    println!("  Modified: {:?}", metadata.modified());

    Ok(())
}

fn view_directory_properties(dirname: &str) -> io::Result<()> {
    let path = Path::new(dirname);
    let metadata = fs::metadata(path)?;

    println!("Directory properties for '{}':", dirname);
    println!("  Type: {:?}", metadata.file_type());
    println!("  Size: {} bytes", metadata.len());
    println!("  Permissions: {:?}", metadata.permissions());
    println!("  Modified: {:?}", metadata.modified());

    Ok(())
}

fn main() -> io::Result<()> {
    create_text_file("new_file.txt")?;
    println!("Created 'new_file.txt'");
    
    create_directory("new_directory")?;
    println!("Created directory 'new_directory'");
    
    copy_file("new_file.txt", "new_file_copy.txt")?;
    println!("Copied 'new_file.txt' to 'new_file_copy.txt'");
    
    create_directory("source_directory")?;
    create_text_file("source_directory/file1.txt")?;
    create_directory("source_directory/nested_directory")?;
    create_text_file("source_directory/nested_directory/file2.txt")?;
    println!("Created source directory structure");
    
    copy_directory_recursive("source_directory", "destination_directory")?;
    println!("Copied 'source_directory' to 'destination_directory' recursively");
    
    match search_file(".", "new_file.txt")? {
        Some(path) => println!("Found 'new_file.txt' at '{}'", path.display()),
        None => println!("'new_file.txt' not found"),
    }
    
    delete_file("new_file_copy.txt")?;
    println!("Deleted 'new_file_copy.txt'");
    
    delete_directory("new_directory")?;
    println!("Deleted directory 'new_directory'");
    
    delete_directories(&["destination_directory/nested_directory", "destination_directory"])?;
    println!("Deleted multiple directories");
    delete_directory("source_directory")?;
    
    list_directory_contents(".")?;
    
    view_file_properties("new_file.txt")?;
    
    view_directory_properties(".")?;

    delete_file("new_file.txt")?;

    Ok(())
}