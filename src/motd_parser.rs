use serde_json::{Value, json};
use regex::Regex;

/*
 * minecraft motd parser
 * (c) 2023 Kevin Zheng
 * Released under the MIT license
 */
// This parser is based of the parser from https://github.com/SnowFireWolf/minecraft-motd-parser
// This file -> https://github.com/SnowFireWolf/minecraft-motd-parser/blob/b9d0b90ac93b8632422dd4fc30a5bca754bd72ed/src/motdParser.ts
pub enum Extras { K, L, M, N, O, R }

impl Extras {
  /// This function returns a string value based on the input enum value.
  /// 
  /// Returns:
  /// 
  /// A string slice (`&'static str`) is being returned based on the variant of the `Extras` enum that
  /// `self` matches with. The string slice returned corresponds to the CSS style associated with the
  /// matched variant.
  pub fn get(&self) -> &'static str {
    match self {
      Extras::K => "obfuscated;",
      Extras::L => "font-weight: bold;",
      Extras::M => "text-decoration: line-through;",
      Extras::N => "text-decoration: underline;",
      Extras::O => "font-style: italic;",
      Extras::R => "color: inherit;text-decoration: none !important;font-weight:normal!important;font-style: normal!important;",
    }
  }

  /// The function takes a string symbol as input and returns a corresponding string value based on a
  /// match with predefined values.
  /// 
  /// Arguments:
  /// 
  /// * `symbol`: A reference to a String that represents a Minecraft formatting code symbol.
  /// 
  /// Returns:
  /// 
  /// The function `get_from_symbol` returns a string slice (`&'static str`). If the input `symbol`
  /// matches one of the predefined strings, it returns the corresponding value from the `Extras` enum
  /// using the `get()` method. Otherwise, it returns an empty string (`""`).
  pub fn get_from_symbol(symbol: &String) -> &'static str {
    match symbol.as_str() {
      "§k" => Extras::K.get(),
      "§l" => Extras::L.get(),
      "§m" => Extras::M.get(),
      "§n" => Extras::N.get(),
      "§o" => Extras::O.get(),
      "§r" => Extras::R.get(),
      _ => "",
    }
  }

  /// The function takes a string code and returns a corresponding static string value based on a match
  /// with predefined codes.
  /// 
  /// Arguments:
  /// 
  /// * `code`: A reference to a String that represents a code.
  /// 
  /// Returns:
  /// 
  /// The function `get_from_code` returns a string slice (`&'static str`). It matches the input `code`
  /// parameter with certain strings and returns the corresponding value from the `Extras` enum using
  /// the `get()` method. If the input `code` does not match any of the cases, an empty string is
  /// returned.
  pub fn get_from_code(code: &String) -> &'static str {
    match code.as_str() {
      "k" => Extras::K.get(),
      "l" => Extras::L.get(),
      "m" => Extras::M.get(),
      "n" => Extras::N.get(),
      "o" => Extras::O.get(),
      "r" => Extras::R.get(),
      _ => "",
    }
  }
}

pub enum CodeToHex {
  Black, White,
  DarkBlue, Blue,
  DarkGreen, Green,
  DarkAqua, Aqua,
  DarkRed, Red,
  DarkPurple, LightPurple,
  Gold, Yellow,
  DarkGray, Gray,
}

impl CodeToHex {
  /// This function returns the hexadecimal code for a given color enum value.
  /// 
  /// Returns:
  /// 
  /// This function returns a string that represents the hexadecimal color code associated with the enum
  /// variant of `CodeToHex` that is being matched.
  pub fn get(&self) -> &'static str {
    match self {
      CodeToHex::Black => "#000000",
      CodeToHex::DarkBlue => "#0000AA",
      CodeToHex::DarkGreen => "#00AA00",
      CodeToHex::DarkAqua => "#00AAAA",
      CodeToHex::DarkRed => "#AA0000",
      CodeToHex::DarkPurple => "#AA00AA",
      CodeToHex::Gold => "#FFAA00",
      CodeToHex::Gray => "#AAAAAA",
      CodeToHex::DarkGray => "#555555",
      CodeToHex::Blue => "#5555FF",
      CodeToHex::Green => "#55FF55",
      CodeToHex::Aqua => "#55FFFF",
      CodeToHex::Red => "#FF5555",
      CodeToHex::LightPurple => "#FF55FF",
      CodeToHex::Yellow => "#FFFF55",
      CodeToHex::White => "#FFFFFF",
    }
  }

  /// This function takes a string input representing a color code and returns the corresponding
  /// hexadecimal value.
  /// 
  /// Arguments:
  /// 
  /// * `symbol`: A reference to a String that represents a color code in Minecraft chat formatting.
  /// 
  /// Returns:
  /// 
  /// This function returns a string slice (`&'static str`) representing the hexadecimal color code
  /// associated with the input Minecraft formatting code symbol. If the input symbol is not recognized,
  /// it returns the hexadecimal color code for white.
  pub fn get_from_symbol(symbol: &String) -> &'static str {
    match symbol.as_str() {
      "§0" => CodeToHex::Black.get(),
      "§1" => CodeToHex::DarkBlue.get(),
      "§2" => CodeToHex::DarkGreen.get(),
      "§3" => CodeToHex::DarkAqua.get(),
      "§4" => CodeToHex::DarkRed.get(),
      "§5" => CodeToHex::DarkPurple.get(),
      "§6" => CodeToHex::Gold.get(),
      "§7" => CodeToHex::Gray.get(),
      "§8" => CodeToHex::DarkGray.get(),
      "§9" => CodeToHex::Blue.get(),
      "§a" => CodeToHex::Green.get(),
      "§b" => CodeToHex::Aqua.get(),
      "§c" => CodeToHex::Red.get(),
      "§d" => CodeToHex::LightPurple.get(),
      "§e" => CodeToHex::Yellow.get(),
      "§f" => CodeToHex::White.get(),
      _ => CodeToHex::White.get(),
    }
  }

  /// This function takes a u8 input representing a color code and returns the corresponding
  /// hexadecimal value.
  /// 
  /// Arguments:
  /// 
  /// * `code`: A reference to a u8 that represents a color code in Minecraft chat formatting.
  /// 
  /// Returns:
  /// 
  /// This function returns a string slice (`&'static str`) representing the hexadecimal color code
  /// associated with the input Minecraft formatting code symbol. If the input symbol is not recognized,
  /// it returns the hexadecimal color code for white.
  pub fn get_from_code(code: &u8) -> &'static str {
    match code {
      0 => CodeToHex::Black.get(),
      1 => CodeToHex::DarkBlue.get(),
      2 => CodeToHex::DarkGreen.get(),
      3 => CodeToHex::DarkAqua.get(),
      4 => CodeToHex::DarkRed.get(),
      5 => CodeToHex::DarkPurple.get(),
      6 => CodeToHex::Gold.get(),
      7 => CodeToHex::Gray.get(),
      8 => CodeToHex::DarkGray.get(),
      9 => CodeToHex::Blue.get(),
      10 => CodeToHex::Green.get(),
      11 => CodeToHex::Aqua.get(),
      12 => CodeToHex::Red.get(),
      13 => CodeToHex::LightPurple.get(),
      14 => CodeToHex::Yellow.get(),
      15 => CodeToHex::White.get(),
      _ => CodeToHex::White.get(),
    }
  }
}

pub fn clean_tags(text: &String) -> String {
  let regex = Regex::new(r"(?:§)([0-9a-fA-FklmnorFKLMNOR])").unwrap();
  regex.replace_all(text, "").to_string()
}

fn html_string_formatting(text: &String) -> String {
  text.replace("&", "&amp;")
      .replace("<", "&lt;")
      .replace(">", "&gt;")
      .replace("\"", "&quot;")
      .replace("'", "&#39;")
      .replace("\n", "<br/>")
}

pub fn text_to_html(text: &String) -> String {
  let mut result_html = String::new();
  let mut color_hex = String::new();
  let mut font_style = String::new();

  let regex = Regex::new(r"(?:§)([0-9a-fA-FklmnorFKLMNOR])").unwrap();
  let code_split = regex.split(text);

  for item in code_split {
    let item_lowercase = item.to_lowercase();

    if item_lowercase.len() == 1 {
      color_hex = CodeToHex::get_from_symbol(&item_lowercase).to_string();
    } else if item_lowercase.len() == 2 {
      font_style = Extras::get_from_symbol(&item_lowercase).to_string();
    } else {
      let mut result_color = String::new();
      let mut text_content = item.to_string();

      if color_hex.len() != 0 {
        result_color = format!("color:{};", color_hex);
      }

      if text_content.len() != 0 {
        text_content = html_string_formatting(&text_content);

        if result_color.len() != 0 || font_style.len() != 0 {
          result_html += &format!("<span style=\"{}{}\">{}</span>", result_color, font_style, text_content);
        } else {
          result_html += &text_content;
        }
      }
    }
  }

  result_html
}

pub fn text_to_json(text: &String) -> Value {
  let mut result_json: Value = json!({
    "text": "",
    "extra": []
  });

  let mut color_hex: String = String::new();
  let mut font_style: String = String::new();

  let regex: Regex = Regex::new(r"(?:§)([0-9a-fA-FklmnorFKLMNOR])").unwrap();
  let code_split: regex::Split = regex.split(text);

  for item in code_split {
    let item_lowercase: String = item.to_lowercase();

    if item_lowercase.len() == 1 {
      color_hex = CodeToHex::get_from_symbol(&item_lowercase).to_string();
    } else if item_lowercase.len() == 2 {
      font_style = Extras::get_from_symbol(&item_lowercase).to_string();
    } else {
      let mut inner_json: Value = json!({
        "text": "",
        "extra": []
      });

      if font_style.len() != 0 {
        inner_json[&font_style] = serde_json::Value::Bool(true);
      }

      if item.len() != 0 {
        inner_json["text"] = item.into();
      }

      if color_hex.len() != 0 {
        inner_json["color"] = serde_json::Value::String(String::from(&color_hex));
      }

      if result_json["extra"].is_array() {
        result_json["extra"].as_array_mut().unwrap().push(inner_json);
      }
    }
  }

  result_json
}

pub fn json_to_html(json: &Value) -> String {
  let mut result_html = String::new();
  let mut color_hex = String::new();
  let mut font_style = String::new();

  for key in json.as_object().unwrap().keys() {
    let key_lowercase = key.to_lowercase();

    if key_lowercase.len() == 1 {
      color_hex = CodeToHex::get_from_symbol(&key_lowercase).to_string();
    } else if key_lowercase.len() == 2 {
      font_style = Extras::get_from_symbol(&key_lowercase).to_string();
    } else {
      let mut inner_html = String::new();

      if font_style.len() != 0 {
        inner_html += &format!("<span style=\"{}\">", font_style);
      }

      if json[key].is_string() {
        inner_html += &text_to_html(&json[key].as_str().unwrap().to_string());
      } else if json[key].is_number() {
        inner_html += &text_to_html(&json[key].as_u64().unwrap().to_string());
      }

      if font_style.len() != 0 {
        inner_html += "</span>";
      }

      if color_hex.len() != 0 {
        inner_html = format!("<span style=\"{}\">{}</span>", color_hex, inner_html);
      }

      result_html += &inner_html;
    }
  }

  result_html
}

pub fn json_render(json: &Value) -> String {
  json_to_html(json)
}
