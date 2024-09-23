<template>
  <v-app-bar app color="primary" density="compact" relative>
    <v-app-bar-title>
      HERMES
      <span class="d-none d-sm-inline-block">- {{ $t('app.slogan') }}</span>
    </v-app-bar-title>

    <template #append>
      <robot-status-switcher class="align-self-center" />

      <v-divider vertical inset opacity="0.5" />

      <lan-control />

      <v-tooltip location="bottom">
        <template #activator="{ props }">
          <v-btn v-bind="props" :active="false" :to="{ name: 'settings' }" icon="mdi-cog" />
        </template>
        <span>{{ t('settings') }}</span>
      </v-tooltip>

      <v-divider vertical inset opacity="0.5" />

      <v-tooltip location="bottom">
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            href="https://github.com/dclause/hermes-studio"
            target="_blank"
            icon="mdi-github"
          />
        </template>
        <span>{{ t('github') }}</span>
      </v-tooltip>
    </template>

    <app-loader />
  </v-app-bar>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<i18n>
{
  "en": {
    "settings": "Settings",
    "github": "Github"
  },
  "fr": {
    "settings": "Paramètres",
    "github": "Github"
  }
}
</i18n>
