use std::{ net::TcpListener, fs::File, io::{ BufReader, Read }, path::{ Path }, env };
use base64::{ Engine, engine::general_purpose };
use simplelog::{ error, debug, trace };
use serde_json::{ Value };

use crate::motd_parser::{ clean_tags, text_to_json, text_to_html };
use crate::constants::DEFAULT_FAV_ICON_STRING;
use crate::settings::Settings;

pub fn is_port_taken(port: u16) -> bool {
  match TcpListener::bind(format!("127.0.0.1:{}", port)) {
    Ok(listener) => {
      debug!("Port {} is not taken", port);
      drop(listener);
      false
    }
    Err(_) => {
      debug!("Port {} is taken", port);
      true
    }
  }
}

pub fn is_in_dev() -> bool {
  match env::var("ENV") {
    Ok(_) => true,
    Err(_) => false,
  }
}

fn make_absolute_path(path: &String) -> String {
  let path = Path::new(&path);

  if path.is_relative() {
    let current_dir = env::current_dir().ok().unwrap();
    let absolute_path = current_dir.join(path);
    String::from(absolute_path.to_str().unwrap())
  } else {
    String::from(path.to_path_buf().to_str().unwrap())
  }
}

pub fn get_fav_icon(settings: &Settings) -> String {
  if settings.favIcon.is_some() {
    return settings.favIcon.clone().unwrap();
  }
  if settings.favIconPath.is_some() {
    let path: String = make_absolute_path(&settings.favIconPath.clone().unwrap());
    let file: Result<File, std::io::Error> = File::open(path);
    match file {
      Ok(file) => {
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        match reader.read_to_end(&mut buffer) {
          Ok(_) => {
            let base64: String = general_purpose::URL_SAFE.encode(&buffer);
            let favicon: String = format!("data:image/png;base64,{}", base64);
            trace!("FavIcon base64: {}", favicon);
            return favicon;
          }
          Err(e) => {
            error!("Failed to read favIconPath: {}", e);
            return String::from(DEFAULT_FAV_ICON_STRING.clone());
          }
        }
      }
      Err(e) => {
        error!("Failed to open favIconPath: {}", e);
        return String::from(DEFAULT_FAV_ICON_STRING.clone());
      }
    }
  }
  return String::from(DEFAULT_FAV_ICON_STRING.clone());
}

pub enum MotdOutputType {
  JSON,
  HTML,
  PlainText,
}

pub enum MotdOutput {
  JSON(Value),
  HTML(String),
  PlainText(String),
}

pub fn get_motd(settings: &Settings, output_type: MotdOutputType) -> Option<MotdOutput> {
  // TODO: Implement this
  match output_type {
    MotdOutputType::JSON => Some(MotdOutput::JSON(text_to_json(&settings.serverName))),
    MotdOutputType::PlainText => Some(MotdOutput::PlainText(clean_tags(&settings.serverName))),
    MotdOutputType::HTML => Some(MotdOutput::HTML(text_to_html(&settings.serverName))),
  }
}

pub enum ServerStatus {
  Sleeping,
  Running,
  Starting,
  Stopped,
}