import '@mdi/font/css/materialdesignicons.css';
import 'vuetify/styles';
import { useI18n } from 'vue-i18n';
import { createVuetify } from 'vuetify';
import { createVueI18nAdapter } from 'vuetify/locale/adapters/vue-i18n';
import i18n from '@/plugins/i18n';

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
  locale: {
    adapter: createVueI18nAdapter({ i18n, useI18n }),
  },
  theme: {
    defaultTheme: 'OffModeTheme',
    variations: {
      colors: ['primary'],
      lighten: 2,
      darken: 0,
    },
    themes: {
      RealTimeModeTheme: {
        dark: false,
        colors: {
          'timeline-border': '#737070',
        },
      },
      OffModeTheme: {
        dark: false,
        colors: {
          primary: '#48a9a6',
          'timeline-border': '#737070',
        },
      },
    },
  },
});
