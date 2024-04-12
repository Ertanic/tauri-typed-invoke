//! A small utility that generates a typescript declaration file for the [`invoke`] function 
//! from functions found in code by Tauri [commands].
//! Thanks to this, there is no mistaking the name of the command.
//! 
//! # Example
//! 
//! **main.rs:**
//! 
//! ```rust
//! fn main() {
//!     tauri::Builder::default()
//!         .invoke_handler(generate_handler![get_weather, get_config])
//!         .run(tauri::generate_context!())
//!         .expect("error while running tauri application");
//! }
//! 
//! #[tauri::command]
//! fn get_weather() -> String {
//!     "sunny".to_string()
//! }
//! // or
//! use tauri::command;
//! #[command]
//! fn get_config() -> String {
//!     "config".to_string()
//! }
//! ```
//! 
//! **build.rs:**
//! 
//! ```rust
//! fn main() {
//!     tauri_named_invoke::build("ui").unwrap();
//!     tauri_build::build();
//! }
//! ```
//! 
//! The file will be generated at the following path:
//! 
//! ```shell
//! project root
//! ├── ui
//! │   └── invoke.d.ts
//! ├── src
//! │   └── main.rs
//! └── Cargo.toml
//! ```
//! 
//! The generated file will contain:
//! 
//! ```typescript
//! import * as tauri from '@tauri-apps/api/tauri';
//! declare module '@tauri-apps/api' {
//!     type Commands = 
//!           'get_weather'
//!         | 'get_config';
//!     function invoke<T>(cmd: Commands, args?: InvokeArgs): Promise<T>;
//! }
//! ```
//! 
//! [`invoke`]: https://tauri.app/v1/api/js/tauri/#invoke
//! [commands]: https://docs.rs/tauri/1.6.1/tauri/command/index.html

use std::{env, path::Path};

use glob::glob;
use regex::Regex;

/// Generates an `invoke.d.ts` file declaring [`invoke`] function values composed 
/// of function names labeled with the [`tauri::command`] attribute.
/// 
/// * path - The path to the directory where the `invoke.d.ts` file will be generated.
/// 
/// # Example
/// 
/// ```rust
/// fn main() {
///     tauri_named_invoke::build("ui").unwrap();
/// }
/// ```
/// 
/// The file will be generated at the following path:
/// 
/// ```shell
/// project root
/// ├── ui
/// │   └── invoke.d.ts
/// ├── src
/// │   └── main.rs
/// └── Cargo.toml
/// ```
/// 
/// [`invoke`]: https://tauri.app/v1/api/js/tauri/#invoke
/// [`tauri::command`]: https://docs.rs/tauri/1.6.1/tauri/command/index.html
pub fn build(path: impl AsRef<std::path::Path>) -> Result<(), Box<dyn std::error::Error>> {
    let typed_file = Path::new(env::var("CARGO_MANIFEST_DIR")?.as_str())
        .join(path)
        .join("invoke.d.ts");
    let fn_names = parse_functions();
    std::fs::write(typed_file, get_content(fn_names))?;
    Ok(())
}

fn parse_functions() -> Vec<String> {
    let mut names = Vec::new();

    let rx = Regex::new(r"(?m)\#\[(?:tauri::)?command][\s\w]*fn\s+([\w\d_-]+)").unwrap();
    for file in glob("**/*.rs").unwrap() {
        let file = file.unwrap();
        println!("cargo:rerun-if-changed={}", file.display());
        let content = std::fs::read_to_string(file).unwrap();
        for cap in rx.captures_iter(&content) {
            names.push(cap[1].to_string());
        }
    }

    names
}

fn get_content(names: Vec<String>) -> String {
    let names = names
        .iter()
        .map(|f| format!("'{}'", f))
        .collect::<Vec<_>>()
        .join("\n\t\t| ");

    format!(
"import * as tauri from '@tauri-apps/api/tauri';
declare module '@tauri-apps/api/tauri' {{
    type Commands = 
\t\t  {};
    function invoke<T>(cmd: Commands, args?: InvokeArgs): Promise<T>;
}}", names)
}