use simplelog::{ error, info };
use serde::{ Serialize };
use reqwest::Client;

use crate::settings::Settings;

#[derive(Serialize)]
struct __DiscordEmbeds {
  title: String,
  color: u32,
}

#[derive(Serialize)]
struct DiscordContent {
  content: Option<()>,
  embeds: Vec<__DiscordEmbeds>,
  username: String,
  avatar_url: String,
}

pub struct SleepingDiscord {
  settings: Settings,
  client: Client,
}

impl SleepingDiscord {
  pub fn new(settings: Settings) -> Self {
    Self {
      settings,
      client: Client::new(),
    }
  }

  async fn send_message(&self, content: DiscordContent, woke: bool) {
    if woke {
      info!("[Discord] Sending waking up message");
    } else {
      info!("[Discord] Sending closing server message");
    }

    if let Some(discord_webhook_url) = &self.settings.discordWebhookUrl {
      let response: Result<reqwest::Response, reqwest::Error> = self.client
        .post(discord_webhook_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&content).unwrap())
        .send().await;

      if let Ok(response) = response {
        if let Ok(text) = response.text().await {
          info!("[Discord] response: {}", text);
        }
        else {
          error!("[Discord] Failed to get response text")
        }
      }
      else if let Err(err) = response {
        error!("[Discord] Failed to send message: {}", err);
      }
    }
  }

  pub async fn on_player_logging(&self, player_name: &str) {
    let content: DiscordContent = DiscordContent {
      content: None,
      embeds: vec![__DiscordEmbeds {
        title: format!("⏰ {} woke up the server !", player_name),
        color: 25344,
      }],
      username: "SleepingServerStarter".to_owned(),
      avatar_url: "https://raw.githubusercontent.com/vincss/mcsleepingserverstarter/feature/discord_notification/docs/sleepingLogo.png".to_owned(),
    };

    self.send_message(content, true).await;
  }

  pub async fn on_server_stop(&self) {
    let content: DiscordContent = DiscordContent {
      content: None,
      embeds: vec![__DiscordEmbeds {
        title: "💤 Server has shut down.".to_owned(),
        color: 25344,
      }],
      username: "SleepingServerStarter".to_owned(),
      avatar_url: "https://raw.githubusercontent.com/vincss/mcsleepingserverstarter/feature/discord_notification/docs/sleepingLogo.png".to_owned(),
    };

    self.send_message(content, false).await;
  }
}
