<template>
  <v-layout class="align-center h-100">
    <v-card class="mx-auto pa-5" variant="elevated">
      <div class="align-center d-flex flex-no-wrap">
        <v-avatar class="pa-2" rounded="0" size="125" variant="outlined" border="0">
          <!--          <svg-robot-broken />-->
          <v-icon icon="mdi-robot-confused" size="110" color="primary" />
        </v-avatar>
        <div>
          <v-card-title class="text-h5">
            {{ t('title') }}
          </v-card-title>
          <v-card-subtitle class="text-body-1">
            {{ t('description') }}
            <app-link :to="{ path: '/' }">
              {{ t('link') }}
            </app-link>
            !
          </v-card-subtitle>
        </div>
      </div>
    </v-card>
  </v-layout>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<i18n>
{
  "en": {
    "description": "Better",
    "link": "get back to known territories",
    "title": "Oops ... This page does not exists"
  },
  "fr": {
    "description": "Mieux vaut",
    "link": "revenir en territoire connu",
    "title": "Oops ... Cette page n'existe pas"
  }
}
</i18n>
