//@ts-nocheck

import { addMessages, init, locale } from "svelte-i18n";

const invoke = window.__TAURI__.core.invoke;

// Инициализация с fallback-локалью
init({
  fallbackLocale: "en",
  initialLocale: "en", // Установите начальную локаль
});

export async function loadRustTranslations(lang) {
  try {
    const translations = await invoke("get_translations", {
      langCode: lang,
    });

    addMessages(lang, translations);
    locale.set(lang);
  } catch (e) {
    console.error(`Failed to load locale ${lang}:`, e);
  }
}
