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
      />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useConfigStore } from '@/stores/configurationStore';

const { t } = useI18n();

const languages = [
  { code: 'en', name: 'English' },
  { code: 'fr', name: 'Français' },
];

const configStore = useConfigStore();
const { locale } = storeToRefs(configStore);

watch(locale, (locale, oldLocale) => {
  if (oldLocale && oldLocale != locale) {
    configStore.update({ locale });
  }
});
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
