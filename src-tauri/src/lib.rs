mod modules;
use modules::app_settings::{get_option, get_settings, update_settings};
use modules::i18n::{
    get_current_language, get_languages, get_translations, get_translations_current,
    switch_language,
};
use modules::theming::{get_theme, get_themes};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_current_language,
            get_languages,
            get_translations,
            get_translations_current,
            switch_language,
            get_settings,
            update_settings,
            get_option,
            get_theme,
            get_themes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
