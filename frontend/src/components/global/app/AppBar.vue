<template>
  <v-app-bar app color="primary" density="compact" relative>
    <v-app-bar-title>
      HERMES
      <span class="d-none d-sm-inline-block">- {{ $t('app.slogan') }}</span>
    </v-app-bar-title>

    <template #append>
      <lan-control />

      <v-tooltip v-for="link in appBarMenuLinks" :key="link.id" location="bottom">
        <template #activator="{ props }">
          <v-btn v-bind="{ ...props, ...link }" />
        </template>
        <span>{{ link.label }}</span>
      </v-tooltip>
    </template>

    <app-loader />
  </v-app-bar>
</template>

<script lang="ts" setup>
import type { NavigationItem } from '@/types/menus';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const appBarMenuLinks: NavigationItem[] = [
  {
    href: 'https://github.com/dclause/hermes-studio',
    target: '_blank',
    id: 'github',
    label: t('github'),
    icon: 'mdi-github',
  },
  {
    to: { name: 'settings' },
    id: 'settings',
    label: t('settings'),
    icon: 'mdi-cog',
  },
];
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
