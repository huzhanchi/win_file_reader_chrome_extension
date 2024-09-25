use std::fs;
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;
use std::path::Path;

#[derive(Deserialize)]
struct Command {
    file_path: String,
}

#[derive(Serialize)]
struct Response {
    content: String,
}

fn handle_message(input: &str) -> String {
    let command: Command = serde_json::from_str(input).unwrap();
    
    let content = match fs::read_to_string(&command.file_path) {
        Ok(content) => content,
        Err(_) => String::from("Error: Unable to read file"),
    };

    let response = Response { content };
    serde_json::to_string(&response).unwrap()
}

fn register_native_messaging_host() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software")
        .join("Google")
        .join("Chrome")
        .join("NativeMessagingHosts")
        .join("com.example.nativeapp");
    let (key, _) = hkcu.create_subkey(&path)?;

    let extension_dir = std::env::current_exe()?
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?
        .to_path_buf();
    let json_path = extension_dir.join("com.example.nativeapp.json");
    let json_path_str = json_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Invalid path to string conversion")
    })?;

    key.set_value("", &json_path_str)?;

    println!("Native app JSON path: {}", json_path_str);
    Ok(())
}

fn main() -> io::Result<()> {
    match register_native_messaging_host() {
        Ok(_) => {
            println!("Native messaging host registered successfully");
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let path = Path::new("Software")
                .join("Google")
                .join("Chrome")
                .join("NativeMessagingHosts")
                .join("com.example.nativeapp");
            
            if let Ok(key) = hkcu.open_subkey(&path) {
                if let Ok(value) = key.get_value::<String, _>("") {
                    println!("Registry key set successfully. Path: {}, Value: {}", path.to_str().unwrap(),value);
                } else {
                    println!("Failed to read registry key value");
                }
            } else {
                println!("Failed to open registry key");
            }
        },
        Err(e) => println!("Failed to register native messaging host: {:?}", e),
    }

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut size_buffer = [0u8; 4];
        stdin.read_exact(&mut size_buffer).unwrap();
        let size = u32::from_ne_bytes(size_buffer);

        let mut message = vec![0u8; size as usize];
        stdin.read_exact(&mut message).unwrap();

        let input = String::from_utf8(message).unwrap();

        println!("The input is {}",input);

        let output = handle_message(&input);

        let output_bytes = output.into_bytes();
        let output_size = output_bytes.len() as u32;
        stdout.write_all(&output_size.to_ne_bytes()).unwrap();
        stdout.write_all(&output_bytes).unwrap();
        stdout.flush().unwrap();
    }
}
