import messages from '@intlify/unplugin-vue-i18n/messages';
import { createI18n } from 'vue-i18n';
import { en as vuetifyEn, fr as vuetifyFr } from 'vuetify/locale';

// https://vue-i18n.intlify.dev/api/general.html#createi18n
export default createI18n({
  globalInjection: true,
  legacy: false,
  allowComposition: true,
  locale: navigator.language.split('-')[0],
  fallbackLocale: 'en',
  messages: {
    fr: { $vuetify: vuetifyFr },
    en: { $vuetify: vuetifyEn },
    ...messages,
  },
});
