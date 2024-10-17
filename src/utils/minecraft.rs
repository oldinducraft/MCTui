use std::iter::once;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use tokio::process::Command;
use walkdir::WalkDir;

use crate::constants::{GAME_VERSION, ORGANIZATION, YGGDRASIL_DOMAIN};

pub struct Minecraft;

impl Minecraft {
    fn main_jar(minecraft_folder: &Path) -> PathBuf {
        minecraft_folder.join(
            ["versions", GAME_VERSION, &format!("{}.jar", GAME_VERSION)]
                .iter()
                .collect::<PathBuf>(),
        )
    }

    fn classpath_with_main_jar(minecraft_folder: &Path) -> impl Iterator<Item = String> {
        let libs_folder = minecraft_folder.join("libraries");
        let paths = WalkDir::new(libs_folder).into_iter();

        paths
            .filter_map(|path| Some(path.ok()?.path().to_path_buf()))
            .filter(|path| path.is_file())
            .chain(once(Minecraft::main_jar(minecraft_folder)))
            .map(|path| path.to_str().unwrap().to_string())
    }

    pub fn java_cmd(client_folder: &Path, access_token: String, username: String, uuid: String) -> Command {
        let minecraft_folder = &client_folder.join("minecraft");
        let authlib_injector_jar = &client_folder.join("authlib.jar");

        let mut cp = Minecraft::classpath_with_main_jar(minecraft_folder);

        let mut cmd = Command::new("java");
        cmd.current_dir(client_folder)
            .args(Minecraft::java_args(minecraft_folder, authlib_injector_jar))
            .args(["-cp", &cp.join(":")])
            .arg("net.minecraft.client.main.Main")
            .args(Minecraft::minecraft_args(
                minecraft_folder,
                access_token,
                username,
                uuid,
            ));
        cmd
    }

    fn java_args(minecraft_folder: &Path, authlib_injector_jar: &Path) -> Vec<String> {
        let natives_directory = minecraft_folder.join("natives").to_str().unwrap().to_string();

        vec![
            "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump".to_string(),
            "-Xss1M".to_string(),
            format!("-Djava.library.path={natives_directory}"),
            format!("-Djna.tmpdir={natives_directory}"),
            format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={natives_directory}"),
            format!("-Dio.netty.native.workdir={natives_directory}"),
            format!("-Dminecraft.launcher.brand={ORGANIZATION}"),
            format!("-Dminecraft.launcher.version={GAME_VERSION}"),
            format!(
                "-javaagent:{}={YGGDRASIL_DOMAIN}",
                authlib_injector_jar.to_str().unwrap()
            ),
            "-Dauthlibinjector.noShowServerName".to_string(),
        ]
    }

    fn minecraft_args(minecraft_folder: &Path, access_token: String, username: String, uuid: String) -> Vec<String> {
        let assets_folder = minecraft_folder.join("assets");

        vec![
            "--gameDir".to_string(),
            minecraft_folder.to_str().unwrap().to_string(),
            "--assetsDir".to_string(),
            assets_folder.to_str().unwrap().to_string(),
            "--version".to_string(),
            GAME_VERSION.to_string(),
            "--assetIndex".to_string(),
            GAME_VERSION.to_string(),
            "--userType".to_string(),
            "mojang".to_string(),
            "--versionType".to_string(),
            "release".to_string(),
            "--quickPlayMultiplayer".to_string(),
            YGGDRASIL_DOMAIN.to_string(),
            "--accessToken".to_string(),
            access_token,
            "--username".to_string(),
            username,
            "--uuid".to_string(),
            uuid,
        ]
    }
}
