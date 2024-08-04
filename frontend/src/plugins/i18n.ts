import messages from '@intlify/unplugin-vue-i18n/messages';
import { createI18n } from 'vue-i18n';

// https://vue-i18n.intlify.dev/api/general.html#createi18n
export default createI18n({
  legacy: false,
  allowComposition: true,
  locale: navigator.language.split('-')[0],
  fallbackLocale: 'en',
  messages: messages,
});
