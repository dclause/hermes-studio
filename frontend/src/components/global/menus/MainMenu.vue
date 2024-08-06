<template>
  <v-list class="text-center" nav>
    <v-tooltip v-for="link in mainMenuLinks" :key="link.id">
      <template #activator="{ props }">
        <v-list-item
          :active="isRouteActive(link)"
          :to="link.to"
          style="font-size: 1.2em"
          v-bind="props"
        >
          <v-icon :icon="link.icon" width="40" />
        </v-list-item>
      </template>
      <span>{{ link.label }}</span>
    </v-tooltip>
  </v-list>
</template>

<script lang="ts" setup>
import type { NavigationItem } from '@/types/menus';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import router from '@/plugins/router';

const { t } = useI18n();
const route = useRoute();
const isRouteActive = (link: NavigationItem) => {
  return router.resolve(link.to ?? link.href ?? '').fullPath === route.fullPath;
};

const mainMenuLinks: NavigationItem[] = [
  {
    to: { name: 'board.list' },
    id: 'board.list',
    label: t('board.list'),
    icon: 'mdi-connection',
  },
];
</script>

<i18n>
{
  "en": {
    "board.list": "Hardware configuration"
  },
  "fr": {
    "board.list": "Configuration matérielle"
  }
}
</i18n>
