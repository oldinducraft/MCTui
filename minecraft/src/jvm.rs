use std::collections::HashMap;

use futures::future::join_all;
use itertools::Itertools;
use strfmt::strfmt;

use crate::game_args::GameArgs;
use crate::manifest::Manifest;
use crate::traits::path::ResolvePath;

#[cfg(target_os = "windows")]
const JAVA_CP_SEPARATOR: &str = ";";
#[cfg(target_os = "linux")]
const JAVA_CP_SEPARATOR: &str = ":";

pub struct Jvm {
    jvm_args:   Vec<String>,
    game_args:  Vec<String>,
    main_class: String,
}

impl Jvm {
    /// Panics:
    /// - if there's an error in JVM args templates. See [`Manifest::jvm`],
    ///   [`strfmt::strfmt`]
    /// - if failed to canonicalize path to authlib, any of libraries. See
    ///   [`dunce::canonicalize`], [`Manifest::libraries`],
    ///   [`Manifest::authlib`]
    /// - if failed to convert canonicalized path to str. See
    ///   [`std::path::Path::to_str`]
    pub async fn new(manifest: &Manifest, args: GameArgs, yggdrasil_domain: &str) -> Jvm {
        assert_eq!(manifest.r#type, "release", "Only release types are supported");

        let jvm_args = Jvm::prepare_jvm_args(manifest, yggdrasil_domain).await;
        let game_args = Jvm::game_args(manifest, args);

        Jvm {
            jvm_args,
            game_args,
            main_class: manifest.main_class.clone(),
        }
    }

    async fn prepare_jvm_args(manifest: &Manifest, yggdrasil_domain: &str) -> Vec<String> {
        let mut vars = HashMap::new();
        vars.insert(
            "natives_directory".to_string(),
            manifest.natives_root.to_str().unwrap().to_string(),
        );
        vars.insert("launcher_name".to_string(), manifest.name.clone());
        vars.insert("launcher_version".to_string(), manifest.version.clone());
        vars.insert("classpath".to_string(), Jvm::classpath(manifest).await);

        let mut args: Vec<String> = manifest
            .jvm
            .iter()
            .map(|arg| strfmt(arg.arg(), &vars).unwrap())
            .collect();

        let authlib = manifest
            .authlib
            .resolve(&manifest.root)
            .await
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        args.push(format!("-javaagent:{authlib}={yggdrasil_domain}"));

        args
    }

    async fn classpath(manifest: &Manifest) -> String {
        let futures = manifest
            .libraries
            .iter()
            .map(|library| library.resolve(&manifest.libraries_root));
        let mut resolved_paths = join_all(futures).await;

        resolved_paths.push(manifest.client.resolve(&manifest.versions_root).await);

        resolved_paths
            .into_iter()
            .map(|path| {
                dunce::canonicalize(path.unwrap())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .join(JAVA_CP_SEPARATOR)
    }

    fn game_args(manifest: &Manifest, args: GameArgs) -> Vec<String> {
        vec![
            "--gameDir".to_string(),
            manifest.root.to_str().unwrap().to_string(),
            "--assetsDir".to_string(),
            manifest.assets_root.to_str().unwrap().to_string(),
            "--version".to_string(),
            manifest.version.clone(),
            "--assetIndex".to_string(),
            manifest.version.clone(),
            "--userType".to_string(),
            args.player.user_type,
            "--versionType".to_string(),
            manifest.r#type.clone(),
            "--quickPlayMultiplayer".to_string(),
            args.quick_play_multiplayer,
            "--accessToken".to_string(),
            args.player.access_token,
            "--username".to_string(),
            args.player.username,
            "--uuid".to_string(),
            args.player.uuid,
        ]
    }

    /// Returns the command to start the game
    /// java <jvm_args> <main_class> <game_args>
    pub fn command(&self) -> Vec<String> {
        let mut command = vec!["java".to_string()];
        command.extend(self.jvm_args.clone());
        command.push(self.main_class.clone());
        command.extend(self.game_args.clone());
        command
    }
}
