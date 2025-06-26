use crate::modules::app_settings::{load_settings, save_settings};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn flatten_json_with_prefix(value: &Value, prefix: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Value::Object(obj) = value {
        for (k, v) in obj {
            let key = if prefix.is_empty() {
                k.to_lowercase()
            } else {
                format!("{}.{k}", prefix).to_lowercase()
            };
            match v {
                Value::Object(_) => {
                    let nested = flatten_json_with_prefix(v, &key);
                    map.extend(nested);
                }
                Value::String(s) => {
                    map.insert(key, s.to_string());
                }
                _ => {}
            }
        }
    }
    map
}

#[tauri::command]
pub fn get_languages() -> Vec<HashMap<String, String>> {
    let mut langs = Vec::new();
    let paths = fs::read_dir("locales").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let code = path.file_stem().unwrap().to_str().unwrap().to_string();
            let data = fs::read_to_string(&path).unwrap();
            let json: Value = serde_json::from_str(&data).unwrap();
            let name = {
                let name_ref = json
                    .get("lang_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(code.as_str());
                name_ref.to_string()
            };
            langs.push(HashMap::from([
                ("code".to_string(), code),
                ("name".to_string(), name),
            ]));
        }
    }
    langs
}

#[tauri::command]
pub fn get_current_language() -> String {
    let settings = load_settings();
    settings.language
}

#[tauri::command]
pub fn switch_language(new_lang: String) -> Result<(), String> {
    println!("Switching language to: {}", new_lang);
    let mut settings = load_settings();
    settings.language = new_lang;
    save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_translations_current() -> HashMap<String, String> {
    let settings = load_settings();
    let lang_code = settings.language;
    let path = format!("locales/{}.json", lang_code);
    let data = fs::read_to_string(path).unwrap_or_default();
    let json: Value = serde_json::from_str(&data).unwrap_or(Value::Null);
    flatten_json_with_prefix(&json, "")
}

#[tauri::command]
pub fn get_translations(lang_code: String) -> HashMap<String, String> {
    let path = format!("locales/{}.json", lang_code);
    let data = fs::read_to_string(path).unwrap_or_default();
    let json: Value = serde_json::from_str(&data).unwrap_or(Value::Null);
    flatten_json_with_prefix(&json, "")
}
