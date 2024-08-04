<template>
  <v-navigation-drawer app clipped mini-variant permanent rail>
    <v-tooltip>
      <template #activator="{ props }">
        <v-avatar class="d-block text-center mx-auto mt-4 pa-1" color="" size="60" v-bind="props">
          <router-link to="/">
            <svg-robot title="test" />
          </router-link>
        </v-avatar>
      </template>
      <span>{{ $t('app.slogan') }}</span>
    </v-tooltip>
    <v-divider class="mx-3 my-5" />

    <main-menu />
  </v-navigation-drawer>
</template>

<script lang="ts" setup></script>
