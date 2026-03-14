import { addMessages, init, getLocaleFromNavigator, locale } from 'svelte-i18n';
import en from './locales/en.json';
import ru from './locales/ru.json';

export const SUPPORTED_LOCALES = ['en', 'ru'];

export function setupI18n() {
  addMessages('en', en);
  addMessages('ru', ru);

  const saved = typeof localStorage !== 'undefined' ? localStorage.getItem('locale') : null;

  init({
    fallbackLocale: 'en',
    initialLocale: saved || getLocaleFromNavigator()?.split('-')[0] || 'en',
  });
}

export function setLocale(lang) {
  locale.set(lang);
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('locale', lang);
  }
}

/** Map an OS locale string (e.g. "ru-RU", "en-US") to a supported locale code. */
/** @param {string | null | undefined} osLocale */
export function resolveLocale(osLocale) {
  const lang = osLocale?.split('-')[0]?.toLowerCase() ?? '';
  return SUPPORTED_LOCALES.includes(lang) ? lang : 'en';
}

export { locale };
