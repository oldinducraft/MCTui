# Oldinducraft Minecraft Launcher  

A TUI-based Minecraft launcher that downloads the Minecraft client and Authlib-injector from specified URL, performs authorization, and launches Minecraft

## Configuration  

__⚠️Outdated, will be updated soon⚠️__

The following variables can be modified in the `src/constants.rs` file:  

| Variable | Description |
| --- | --- |
| `YGGDRASIL_DOMAIN` | URL of the Yggdrasil server, without scheme and trailing slash. Also used as the server URL for Quickplay |
| `CLIENT_URL` | URL of a `.tar.gz` archive containing the Minecraft client and the Authlib-injector JAR at the root |
| `HASH_URL` | URL of a `.json` file containing the SHA-256 hash of each file in the `.tar.gz` archive¹ |
| `CONFIG_FILENAME` | Name of the config file² |
| `CLIENT_ARCHIVE_FILENAME` | Name of the archive file³ |
| `CLIENT_FOLDER_NAME` | Name of the folder that contains the unpacked Minecraft client³ |
| `QUALIFIER` | Reverse domain notation of the application, excluding the organization or application name⁴ |
| `ORGANIZATION` | Name of the organization developing or commissioning this application⁴ |
| `APPLICATION` | Name of the application itself⁴ |
| `GAME_VERSION` | Version of the downloaded Minecraft client. Used for setting `-Dminecraft.launcher.version`, `--version`, and `--assetIndex`. It is also used as the name of the `.jar` file in `minecraft/versions/`, e.g.: `minecraft/versions/<GAME_VERSION>.jar` |

¹Example `HASH_URL` File:  

```json
{
    "minecraft/assets/objects/ad/ad17cd0f0bd7e8810f0517ae2096fe825a574518": "509212d3349be1cd117db7f6f1fc8e0aabb6c1ecb3cbc25717ff4392031582d0",
    "minecraft/assets/objects/ad/ad92224110bbda3cf08f9512f61831f112255517": "16de5d2bba71b1493b63cfd0ef6e5253c94d1aa5d38492a5371e967e5475649c",
    "minecraft/assets/objects/ad/ad7d770b7fff3b64121f75bd60cecfc4866d1cd6": "062c3175b3ab2b7e44b52a10bb9ff60eb6c01f5b19f165e90df0ab0c215b07d6",
    "minecraft/assets/objects/ad/add752f78fbd0398f0e9e0755e4e12dc196b821c": "af45aaa746c6c384ffc9c9ae655e68b03476b549fcec04987bd5959b7d86cc70",
    "minecraft/assets/objects/ad/ad931f657e9bac3544cce94918956a5c88315d71": "d8195325db995e135694c2a696b424d7e08225186761018b3ef8f3e96d7ce243",
    "minecraft/assets/objects/ad/ad98c472dafa5b154655754eb985a5ad8a6a18ca": "cdd7fb90c0eba4c15be7f42636348f0de02b3cd14b9d44e134c0a2812ecebab7",
    "minecraft/assets/objects/ad/ad645926282faa862f27ebda44549e265cc91694": "b8aa12e17d9257627ed2165221902275b8fbfe7e9ab5e4de1a94637acfd1878f",
    // ...
}
```  

²Stored in [the config directory](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.config_dir)

³Stored in [the data directory](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.data_dir)

⁴More information: [ProjectDirs Documentation](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.from)

## Build  

Ensure you are using `cargo` v1.81.0 and `rustc` v1.81.0.  

To compile the project, follow these steps:  

1. Install the latest stable Rust toolchain: `rustup install stable`  
2. Set the latest stable version as the default toolchain: `rustup default stable`  
3. Add the Windows target: `rustup target add x86_64-pc-windows-gnu`  
4. Add the Linux target: `rustup target add x86_64-unknown-linux-gnu`  
5. Install the Windows linker: `apt-get install mingw-w64`  
6. Build the project for both targets: `cargo build --release`  

<!-- ## Screenshots

![Screenshot 2024-10-14 181818](https://github.com/user-attachments/assets/f90a7243-cc86-4200-a6b1-4586fc6b7b7d)
![Screenshot 2024-10-14 181824](https://github.com/user-attachments/assets/5f55dc36-cf91-4cc3-8b10-d65ea5271ca3)
![Screenshot 2024-10-14 181844](https://github.com/user-attachments/assets/24f3a1b0-3b11-4cfc-955d-13ee6eb66987)
![Screenshot 2024-10-14 181908](https://github.com/user-attachments/assets/cb55c61b-4480-4a14-8034-e06143e71ed1)
![Screenshot 2024-10-14 181930](https://github.com/user-attachments/assets/e5ea4cd4-1c34-4ebb-881f-041b358ae59d) -->
