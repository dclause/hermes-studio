<template>
  <v-card class="mx-auto my-8" variant="flat">
    <v-card-item>
      <v-card-title>{{ t('title') }}</v-card-title>
    </v-card-item>

    <v-card-text>
      <v-select
        v-model="locale"
        :hint="t('description')"
        :items="languages"
        item-title="name"
        item-value="code"
        persistent-hint
        variant="underlined"
        @update:model-value="onChange"
      />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useConfigStore } from '@/stores/configuration';

const { t, locale: i18n } = useI18n();
const languages = [
  { code: 'en', name: 'English' },
  { code: 'fr', name: 'Français' },
];

const configStore = useConfigStore();
const locale = ref<string>(configStore.locale);
const onChange = (locale: string) => {
  console.log('OnChange');
  configStore.updateLanguage(i18n, locale);
};
</script>

<i18n>
{
  "en": {
    "title": "Language settings",
    "description": "Select the interface language."
  },
  "fr": {
    "title": "Paramètres de langue",
    "description": "Choisissez la langue de l'interface."
  }
}
</i18n>
