# About

`tauri-named-invoke` is a small utility that generates a typescript declaration file for the [`invoke`](https://tauri.app/v1/api/js/tauri/#invoke) function from functions found in code by Tauri [commands](https://docs.rs/tauri/1.6.1/tauri/command/index.html).
Thanks to this, there is no mistaking the name of the command.

# Example

**main.rs:**

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![get_weather, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_weather() -> String {
    "sunny".to_string()
}
// or
use tauri::command;
#[command]
fn get_config() -> String {
    "config".to_string()
}
```

**build.rs:**

```rust
fn main() {
    tauri_named_invoke::build("ui").unwrap();
    tauri_build::build();
}
```

The file will be generated at the following path:

```shell
project root
├── ui
│   └── invoke.d.ts
├── src
│   └── main.rs
└── Cargo.toml
```

The generated file will contain:

```typescript
import * as tauri from '@tauri-apps/api/tauri';
declare module '@tauri-apps/api' {
    type Commands = 
          'get_weather'
        | 'get_config';

    function invoke<T>(cmd: Commands, args?: InvokeArgs): Promise<T>;
}
``` 