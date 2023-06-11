use std::{ io::{ BufWriter, Write, BufReader, Read }, fs::File, time::{ SystemTime, UNIX_EPOCH } };
use simplelog::{ error, info, debug, warn, trace };
use serde::{ Serialize, Deserialize };
use once_cell::sync::Lazy;
use serde_yaml;

use crate::constants::SETTING_FILE_PATH;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WebServeDynmap {
  Bool(bool),
  String(String),
  None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Version {
  String(String),
  Bool(bool),
  None,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct Settings {
  pub serverName: String,
  pub serverPort: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bedrockPort: Option<u16>,
  pub maxPlayers: u16,
  pub loginMessage: String,
  pub serverOnlineMode: bool,
  pub webPort: u16,
  pub webStopOnStart: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub webServeDynmap: Option<WebServeDynmap>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub webSubPath: Option<String>,
  pub startMinecraft: bool,
  pub minecraftCommand: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub preventStop: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<Version>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub favIcon: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub favIconPath: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub minecraftWorkingDirectory: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub discordWebhookUrl: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blackListAddress: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub whiteListedNames: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hideIpInLogs: Option<bool>,
}

pub static DEFAULT_SETTINGS: Lazy<Settings> = Lazy::new(|| {
  Settings {
    serverName: String::from("A Minecraft Server"),
    serverPort: 25565,
    maxPlayers: 20,
    loginMessage: String::from("Welcome to the server!"),
    serverOnlineMode: true,
    webPort: 0,
    webStopOnStart: false,
    startMinecraft: true,
    minecraftCommand: String::from("java -jar server.jar nogui"),
    version: Some(Version::Bool(false)),
    favIcon: None,
    favIconPath: None,
    minecraftWorkingDirectory: None,
    discordWebhookUrl: None,
    blackListAddress: None,
    whiteListedNames: None,
    hideIpInLogs: None,
    bedrockPort: None,
    preventStop: None,
    webServeDynmap: None,
    webSubPath: None,
  }
});

fn save_default() -> Result<(), ()> {
  let settings = serde_yaml::to_string(&DEFAULT_SETTINGS.clone());
  match settings {
    Ok(settings) => {
      let file = File::create(SETTING_FILE_PATH);
      match file {
        Ok(file) => {
          let mut writer = BufWriter::new(file);
          match writer.write_all(settings.as_bytes()) {
            Ok(_) => {
              info!("Settings file created!");
              return Result::Ok(());
            }
            Err(e) => {
              error!("Failed to write to settings file: {}", e);
              return Result::Err(());
            }
          }
        }
        Err(e) => {
          error!("Failed to create settings file: {}", e);
          return Result::Err(());
        }
      }
    }
    Err(e) => {
      error!("Failed to serialize default settings: {}", e);
      return Result::Err(());
    }
  }
}

fn backup_settings(mut reader: BufReader<File>) -> Result<(), ()> {
  let mut settings: String = String::new();
  match reader.read_to_string(&mut settings) {
    Ok(_) => {
      let backup_file: Result<File, std::io::Error> = File::create(
        format!(
          "{}-invalid-{}.yml",
          SETTING_FILE_PATH.strip_suffix(".yml").unwrap(),
          SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        )
      );
      debug!(
        "Backup file name: {}",
        format!(
          "{}-invalid-{}.yml",
          SETTING_FILE_PATH.strip_suffix(".yml").unwrap(),
          SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        )
      );
      trace!("Backup file contents: {}", settings);
      match backup_file {
        Ok(mut backup_file) => {
          match backup_file.write_all(&settings.as_bytes()) {
            Ok(_) => {
              info!("Settings file backed up!");
              return Result::Ok(());
            }
            Err(e) => {
              error!("Failed to write to settings backup file: {}", e);
              return Result::Err(());
            }
          }
        }
        Err(e) => {
          error!("Failed to create settings backup file: {}", e);
          return Result::Err(());
        }
      }
    }
    Err(e) => {
      error!("Failed to read settings file: {}", e);
      return Result::Err(());
    }
  }
}

pub fn get_settings() -> Settings {
  let settings: Settings = DEFAULT_SETTINGS.clone();
  let file: Result<File, std::io::Error> = File::open(SETTING_FILE_PATH);
  match file {
    Ok(file) => {
      let reader: BufReader<File> = BufReader::new(file);
      let settings_from_files: Result<Settings, serde_yaml::Error> = serde_yaml::from_reader(
        &mut reader.get_ref().try_clone().unwrap()
      );
      match settings_from_files {
        Ok(settings_from_files) => {
          trace!("{:?}", settings_from_files);
          return Settings {
            serverName: settings_from_files.serverName.clone(),
            serverPort: settings_from_files.serverPort,
            bedrockPort: settings_from_files.bedrockPort.or(settings.bedrockPort),
            maxPlayers: settings_from_files.maxPlayers,
            loginMessage: settings_from_files.loginMessage.clone(),
            serverOnlineMode: settings_from_files.serverOnlineMode,
            webPort: settings_from_files.webPort,
            webStopOnStart: settings_from_files.webStopOnStart,
            webServeDynmap: settings_from_files.webServeDynmap.or(settings.webServeDynmap),
            webSubPath: settings_from_files.webSubPath.or(settings.webSubPath),
            startMinecraft: settings_from_files.startMinecraft,
            minecraftCommand: settings_from_files.minecraftCommand.clone(),
            preventStop: settings_from_files.preventStop.or(settings.preventStop),
            version: settings_from_files.version.or(settings.version),
            favIcon: settings_from_files.favIcon.or(settings.favIcon),
            favIconPath: settings_from_files.favIconPath.or(settings.favIconPath),
            minecraftWorkingDirectory: settings_from_files.minecraftWorkingDirectory.or(
              settings.minecraftWorkingDirectory
            ),
            discordWebhookUrl: settings_from_files.discordWebhookUrl.or(settings.discordWebhookUrl),
            blackListAddress: settings_from_files.blackListAddress.or(settings.blackListAddress),
            whiteListedNames: settings_from_files.whiteListedNames.or(settings.whiteListedNames),
            hideIpInLogs: settings_from_files.hideIpInLogs.or(settings.hideIpInLogs),
          };
        }
        Err(e) => {
          error!("Failed to deserialize settings: {}", e);
          warn!("Using default settings");
          match backup_settings(BufReader::new(File::open(SETTING_FILE_PATH).unwrap())) {
            Ok(_) => {
              save_default().unwrap();
            }
            Err(_) => {
              error!("Failed to backup settings");
            }
          }
        }
      }
    }
    Err(e) => {
      error!("Failed to open settings file: {}", e);
      warn!("Using default settings");
      save_default().unwrap();
    }
  }
  settings
}
