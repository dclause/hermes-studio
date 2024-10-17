<template>
  <v-navigation-drawer app clipped mini-variant permanent rail>
    <v-tooltip>
      <template #activator="{ props }">
        <v-avatar
          class="d-block text-center mx-auto mt-4 pa-1"
          color="primary"
          size="50"
          v-bind="props"
        >
          <router-link to="/">
            <!--            <svg-robot title="test" />-->
            <v-icon icon="mdi-robot-love" title="test" size="40" color="white" />
          </router-link>
        </v-avatar>
      </template>
      <span>{{ t('app.slogan') }}</span>
    </v-tooltip>
    <v-divider class="mx-3 my-5" />

    <main-menu />

    <template #append>
      <div class="text-subtitle-2 text-center font-weight-bold">
        {{ semVersion }}
      </div>
    </template>
  </v-navigation-drawer>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
import { version } from '@/../package.json';

const { t } = useI18n();
const semVersion = version;
</script>
