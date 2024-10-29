# Oldinducraft Minecraft Launcher  

A TUI-based Minecraft launcher that downloads the Minecraft client and Authlib-injector, performs authorization, and launches Minecraft

## Configuration  

The following variables can be modified in the `constants/src/lib.rs` file:  

| Variable | Description |
| --- | --- |
| `YGGDRASIL_DOMAIN` | URL of the Yggdrasil server, without scheme and trailing slash. Also used as the server URL for Quickplay |
| `MANIFEST_URL` | URL of a JSON manifest, more about manifest files [here](minecraft/README.md) |
| `CONFIG_FILENAME` | Name of the config file¹ |
| `QUALIFIER` | Reverse domain notation of the application, excluding the organization or application name² |
| `ORGANIZATION` | Name of the organization developing or commissioning this application² |
| `APPLICATION` | Name of the application itself² |

¹Stored in [the config directory](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.config_dir)

²More information: [ProjectDirs Documentation](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.from)

## Build  

Ensure you are using `cargo` v1.81.0 and `rustc` v1.81.0.  

To compile the project, follow these steps:  

1. Install the latest stable Rust toolchain: `rustup install stable`  
2. Set the latest stable version as the default toolchain: `rustup default stable`  
3. Add the Windows target: `rustup target add x86_64-pc-windows-gnu`  
4. Add the Linux target: `rustup target add x86_64-unknown-linux-gnu`  
5. Install the Windows linker: `apt-get install mingw-w64`  
6. Build the project for both targets: `cargo build --release`
