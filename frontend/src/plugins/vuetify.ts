import '@mdi/font/css/materialdesignicons.css';
import 'vuetify/styles';
import { createVuetify } from 'vuetify';

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
  theme: {
    defaultTheme: 'myCustomTheme',
    themes: {
      myCustomTheme: {
        dark: false,
        variables: {
          'timeline-header-height': '36px',
        },
        colors: {
          'timeline-border': '#737070',
          'timeline-header-height': '36px',
        },
      },
    },
  },
});
