# Oldinducraft Minecraft Launcher  

A TUI-based Minecraft launcher that downloads the Minecraft client and Authlib-injector from specified URL, performs authorization, and launches Minecraft.

## Configuration  

The following variables can be modified in the `src/constants.rs` file:  

| Variable | Description |
| --- | --- |
| `YGGDRASIL_DOMAIN` | URL of the Yggdrasil server, without scheme and trailing slash. Also used as the server URL for Quickplay. |
| `CLIENT_URL` | URL of a `.tar.gz` archive containing the Minecraft client and the Authlib-injector JAR at the root. |
| `HASH_URL` | URL of a `.json` file containing the Blake3 hash of each file in the `.tar.gz` archive.* |
| `CONFIG_FILENAME` | Name of the config file.** |
| `CLIENT_ARCHIVE_FILENAME` | Name of the archive file.*** |
| `CLIENT_FOLDER_NAME` | Name of the folder that contains the unpacked Minecraft client.*** |
| `QUALIFIER` | Reverse domain notation of the application, excluding the organization or application name.**** |
| `ORGANIZATION` | Name of the organization developing or commissioning this application.**** |
| `APPLICATION` | Name of the application itself.**** |
| `GAME_VERSION` | Version of the downloaded Minecraft client. Used for setting `-Dminecraft.launcher.version`, `--version`, and `--assetIndex`. It is also used as the name of the `.jar` file in `minecraft/versions/`, e.g.: `minecraft/versions/<GAME_VERSION>.jar` |

*Example `HASH_URL` File:  

```json
{
  "authlib.jar": "bb336955c320b22ae86b05b10ff30b6b7e0357a265448e2a1173bda02e6f03f9",
  "minecraft/assets/indexes/1.21.1.json": "0f68da9555f69cfa27537146598213c5e3cc90931917c007e5f657de8e0bbce6",
  "minecraft/assets/objects/00/000c82756fd54e40cb236199f2b479629d0aca2f": "c43edac36d3a381d79ab34f54ed7b750e293401f7610806793fce2515d70c153",
  "minecraft/assets/objects/00/0013485e9449e914908ca89a8dc1369eff037a26": "cec45f5b5c85bdf9bdf3e5d5287efa5c4208a37d1a6744535a1c031084388049",
  "minecraft/assets/objects/00/0027d19bb0cd9c914d3ac148aeee6316d9786341": "4c0bcf251947280d53ba133c791d0b5a81e697d080550394a7d2cef9958623a8",
  "minecraft/assets/objects/00/00340157e51d29fdeba7258fd880780f766d77dc": "1de50ac97ff91156c7a37932d098bc0008de68857036cd87b582a088f3d3df89",
  "minecraft/assets/objects/00/003409183ca2b8a8a140bfeb6390d1a9edc8c88b": "2c902dc31ebcbc30ec1ced669f6c08f160a7d6d63a70212e2798c8dbc81dbf74"
}
```  

**Stored in [the config directory](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.config_dir).

***Stored in [the data directory](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.data_dir).

****More information: [ProjectDirs Documentation](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.from).

## Build  

Ensure you are using `cargo` v1.81.0 and `rustc` v1.81.0.  

To compile the project, follow these steps:  

1. Install the latest stable Rust toolchain: `rustup install stable`  
2. Set the latest stable version as the default toolchain: `rustup default stable`  
3. Add the Windows target: `rustup target add x86_64-pc-windows-gnu`  
4. Add the Linux target: `rustup target add x86_64-unknown-linux-gnu`  
5. Install the Windows linker: `apt-get install mingw-w64`  
6. Build the project for both targets: `cargo build --release`  
