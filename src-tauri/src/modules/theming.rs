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
pub fn get_themes() -> Vec<String> {
    let mut themes = Vec::new();
    let paths = fs::read_dir("themes").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                themes.push(name.to_string());
            }
        }
    }
    themes
}

#[tauri::command]
pub fn get_theme(name: String) -> HashMap<String, String> {
    let mut result = HashMap::new();

    // Основная тема
    let path = format!("themes/{}.json", name);
    let data = fs::read_to_string(&path).unwrap_or_default();
    let json: Value = serde_json::from_str(&data).unwrap_or(Value::Null);
    result.extend(flatten_json_with_prefix(&json, ""));

    // Подсветка синтаксиса (hl)
    let hl_path = format!("themes/{}_hl.json", name);
    let hl_data = fs::read_to_string(&hl_path)
        .or_else(|_| fs::read_to_string("themes/default_hl.json"))
        .unwrap_or_default();

    if let Ok(hl_json) = serde_json::from_str::<Value>(&hl_data) {
        let hl_map = flatten_json_with_prefix(&hl_json, "hl");
        result.extend(hl_map);
    }

    result
}
