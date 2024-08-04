<template>
  <v-app id="hermes">
    <!-- AppBar: top bar-->
    <app-bar />

    <!-- Drawer: sidebar on the left-->
    <app-drawer />

    <!-- Main: -->
    <v-main class="bg-grey-lighten-3">
      <v-container class="pa-8" fluid style="overflow-x: auto">
        <component :is="route.meta.layoutComponent || 'div'" />
      </v-container>
    </v-main>

    <!-- Helper: sidebar on the right-->
    <v-navigation-drawer color="grey-darken-2" location="right" name="drawer" temporary />

    <app-toaster />
  </v-app>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { useConnectionStore } from '@/stores/connection';

const route = useRoute();
const store = useConnectionStore();
store.open();
</script>
