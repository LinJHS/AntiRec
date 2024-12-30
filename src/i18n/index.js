import { createI18n } from "vue-i18n";
import Cookies from "js-cookie";
import en from "./en.json";
import zh from "./zh-CN.json";

// Defining the messages object that holds localized messages for different languages
const messages = {
  en: en,
  zh: zh,
};

// Function to get the user's preferred language
export function getLanguage() {
  const chooseLanguage = Cookies.get("language");
  if (chooseLanguage) return chooseLanguage;

  // If no language is chosen
  const language = (
    navigator.language || navigator.browserLanguage
  ).toLowerCase(); // Get the browser's language
  const locales = Object.keys(messages); // Get the available locales
  for (const locale of locales) {
    if (language.indexOf(locale) > -1) {
      return locale;
    }
  }
  return "en"; // Default language is English
}


const i18n = createI18n({
  messages,
  locale: getLanguage(),
  allowComposition: true,
  legacy: false,
  globalInjection: true,
});

export default i18n;
