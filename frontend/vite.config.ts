import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite';
import vue from '@vitejs/plugin-vue';
import path from 'node:path';
import { fileURLToPath, URL } from 'node:url';
import Components from 'unplugin-vue-components/vite';
import { defineConfig } from 'vite';
import vueDevTools from 'vite-plugin-vue-devtools';
import Vuetify, { transformAssetUrls } from 'vite-plugin-vuetify';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    VueI18nPlugin({
      include: [path.resolve(__dirname, './src/translations/**')],
    }),
    vue({
      template: { transformAssetUrls },
    }),
    Components({
      dts: 'src/types/components.d.ts',
      globs: ['src/pages/**/*.vue', 'src/components/**/*.vue'],
    }),
    Vuetify({
      autoImport: true,
      styles: {
        configFile: 'src/styles/settings.scss',
      },
    }),
    vueDevTools({
      launchEditor: 'webstorm',
    }),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    port: 5000,
  },
});
