<template>
  <v-app id="hermes">
    <!-- AppBar: top bar-->
    <app-bar />

    <!-- Drawer: sidebar on the left-->
    <app-drawer />

    <!-- Main: -->
    <v-main class="bg-grey-lighten-3">
      <v-container class="page-container pa-8" fluid style="overflow-x: auto">
        <component :is="route.meta.layoutComponent || 'div'" />
      </v-container>
    </v-main>

    <!-- Helper: sidebar on the right-->
    <v-navigation-drawer color="grey-darken-2" location="right" name="drawer" temporary />

    <app-toaster />
  </v-app>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { getCurrentInstance, watch } from 'vue';
import { useRoute } from 'vue-router';
import { useConfigStore } from '@/stores/configurationStore';
import { useConnectionStore } from '@/stores/connectionStore';

const route = useRoute();

// Force connexion of socket to backend.
const connectionStore = useConnectionStore();
connectionStore.open();

// React to language change in config.
const app = getCurrentInstance();
const configStore = useConfigStore();
const { locale } = storeToRefs(configStore);
watch(locale, (locale) => {
  app!.appContext.config.globalProperties.$i18n.locale = locale;
});
</script>

<style lang="scss" scoped>
.page-container {
  height: 100%;
}
</style>
