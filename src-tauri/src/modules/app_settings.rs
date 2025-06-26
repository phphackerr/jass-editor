use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub theme: String,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            theme: "dark".to_string(),
            language: "en".to_string(),
        }
    }
}

fn get_settings_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    let config_dir = PathBuf::from(appdata).join("Jass Editor");
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }
    config_dir.join("settings.json")
}

pub fn load_settings() -> Settings {
    let settings_path = get_settings_path();
    if settings_path.exists() {
        let data = fs::read_to_string(&settings_path).unwrap_or_default();
        let mut user_settings: Settings = serde_json::from_str(&data).unwrap_or_default();
        // Минимальные размеры
        user_settings.width = user_settings.width.max(800);
        user_settings.height = user_settings.height.max(600);

        // Проверяем и добавляем отсутствующие ключи (если структура изменилась)
        let mut updated = false;
        let default = Settings::default();
        if user_settings.theme.is_empty() {
            user_settings.theme = default.theme;
            updated = true;
        }
        if user_settings.language.is_empty() {
            user_settings.language = default.language;
            updated = true;
        }
        // ...можно добавить другие проверки...

        if updated {
            let _ = save_settings(&user_settings);
        }
        user_settings
    } else {
        let default = Settings::default();
        let _ = save_settings(&default);
        default
    }
}

pub fn save_settings(new_settings: &Settings) -> std::io::Result<()> {
    let settings_path = get_settings_path();
    let json = serde_json::to_string_pretty(new_settings)?;
    fs::write(settings_path, json)
}

#[tauri::command]
pub fn get_settings() -> Settings {
    load_settings()
}

#[tauri::command]
pub fn update_settings(
    new_settings: HashMap<String, serde_json::Value>,
) -> Result<Settings, String> {
    println!("=== Начало update_settings ===");
    println!("Входные данные: {:?}", new_settings);

    let mut settings = load_settings();
    println!("Текущие настройки: {:?}", settings);

    for (key, value) in new_settings {
        println!("Обработка поля '{}': {:?}", key, value);
        match key.as_str() {
            "width" => {
                if let Some(v) = value.as_u64() {
                    settings.width = v as u32;
                }
            }
            "height" => {
                if let Some(v) = value.as_u64() {
                    settings.height = v as u32;
                }
            }
            "theme" => match value {
                serde_json::Value::String(s) => {
                    println!("Установка темы из строки: {}", s);
                    settings.theme = s;
                }
                serde_json::Value::Number(n) => {
                    println!("Установка темы из числа: {}", n);
                    settings.theme = n.to_string();
                }
                _ => {
                    println!("Неизвестный тип значения темы: {:?}", value);
                    if let Some(s) = value.as_str() {
                        println!("Преобразование в строку: {}", s);
                        settings.theme = s.to_string();
                    }
                }
            },
            "language" => {
                if let Some(v) = value.as_str() {
                    settings.language = v.to_string();
                }
            }
            _ => println!("Неизвестное поле: {}", key),
        }
    }

    println!("Настройки перед сохранением: {:?}", settings);
    match save_settings(&settings) {
        Ok(_) => println!("Настройки успешно сохранены"),
        Err(e) => {
            println!("Ошибка сохранения: {}", e);
            return Err(e.to_string());
        }
    }

    println!("=== Конец update_settings ===");
    Ok(settings)
}

#[tauri::command]
pub fn get_option(key: String) -> Option<serde_json::Value> {
    let settings = load_settings();
    match key.as_str() {
        "width" => Some(serde_json::json!(settings.width)),
        "height" => Some(serde_json::json!(settings.height)),
        "theme" => Some(serde_json::json!(settings.theme)),
        "language" => Some(serde_json::json!(settings.language)),
        _ => None,
    }
}
