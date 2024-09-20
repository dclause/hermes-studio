<template>
  <v-list class="text-center" nav>
    <v-tooltip v-for="link in mainMenuLinks" :key="link.id">
      <template #activator="{ props }">
        <v-list-item
          :active="isRouteActive(link)"
          :to="link.to"
          :prepend-icon="link.icon"
          v-bind="props"
        />
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
    icon: 'mdi-cog-transfer',
  },
  {
    to: { name: 'position.list' },
    id: 'position.list',
    label: t('position.list'),
    icon: 'mdi-camera-control',
  },
  {
    to: { name: 'animation.list' },
    id: 'animation.list',
    label: t('animation.list'),
    icon: 'mdi-movie-open',
  },
];
</script>

<i18n>
{
  "en": {
    "board.list": "Hardware configuration",
    "position.list": "Robot control",
    "animation.list": "Animations"
  },
  "fr": {
    "board.list": "Configuration matérielle",
    "position.list": "Contrôle du robot",
    "animation.list": "Animations"
  }
}
</i18n>
