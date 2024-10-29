# Manifests

Manifests are used to determine how and where to download game assets and libraries, and how to launch the game.

## Examples

You can take a look at example manifest and assets list in the `examples` directory. Usage examples can also be found there.

## Manifest Structure

Manifests are JSON files with the following fields:

- **`name`**: The name of the launcher, which will be used as `{launcher_name}` in JVM arguments.
- **`version`**: The version of the launcher, which will be used as `{launcher_version}` in JVM arguments.
- **`javaVersion`**: The required Java version to run the game. (**Currently unused**)
- **`type`**: The type of the game version. (**Currently unused**)
- **`mainClass`**: The main class of the game.
- **`jvm`**: A list of JVM argument templates.
- **`assetIndexes`**: Minecraft's original asset index.
- **`assets`**: A stripped-down list of assets used to download necessary files.
- **`client`**: The Minecraft client JAR.
- **`authlib`**: The Authlib-injector JAR.
- **`libraries`**: A list of libraries to download.

### JVM Argument Templates

JVM argument templates include placeholders that are replaced at runtime. Available placeholders:

- **`{natives_directory}`**: Absolute path to the directory where native libraries are located.
- **`{launcher_name}`**: Replaced with the value from the `name` field.
- **`{launcher_version}`**: Replaced with the value from the `version` field.
- **`{classpath}`**: A list of absolute paths to the libraries and the client JAR.

Arguments can be platform-specific using the `platform` field. Supported values: `Windows`, `Linux`, `Osx`.

**Example:**

- Argument applied to all platforms:  
  `"-Xss1M"`
  
- Argument applied to Windows only:  
  `{ "arg": "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump", "platform": "Windows" }`

### Downloads

The following elements are downloads: `assetIndexes`, `assets`, `client`, `authlib`, and each library in the `libraries` list. Each download includes:

- **`sha1`**: SHA-1 hash of the file.
- **`path`**: Logical path to the file, used both as the save path (`<root>/<path>`) and the download URL (`<host>/<path>`).
- **`platform`** (optional): Specifies the platforms for which the file should be downloaded. If not specified, the file is downloaded on all platforms. Supported values: `Windows`, `Linux`, `Osx`.

## Assets

Unlike Minecraft's original asset index, this library uses a simple list of hashes. Each SHA-1 hash represents both the file's logical path and the URL for downloading it.

**Example:**

SHA-1:  
`f2f7f61ccd20a66297fb416c9c4c7eb7f5973463`

- **Path**: `<root>/assets/objects/f2/f2f7f61ccd20a66297fb416c9c4c7eb7f5973463`
- **URL**: `<host>/f2/f2f7f61ccd20a66297fb416c9c4c7eb7f5973463`
