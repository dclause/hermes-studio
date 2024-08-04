<template>
  <v-layout class="align-center h-100">
    <v-card class="mx-auto pa-5">
      <div class="align-center d-flex flex-no-wrap">
        <v-avatar class="pa-2" rounded="0" size="125">
          <svg-robot-broken />
        </v-avatar>
        <div>
          <v-card-title class="text-h5 pt-0 pb-0">
            {{ t('title') }}
          </v-card-title>
          <v-card-subtitle>
            <div class="font-italic mb-4">
              {{ t('description') }}
            </div>
            <div class="text-body-1">
              {{ t('helper_line1') }}<br>
              <i18n-t for="link" keypath="helper_line2" tag="label">
                <app-link :to="{ name: 'settings' }">
                  {{ t('link') }}
                </app-link>
              </i18n-t>
            </div>
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
    "title": "Oops ... the connection was lost",
    "description": "This is ground control to robot... please respond !",
    "helper_line1": "Please make sure your backend is properly up and running",
    "helper_line2": "as well as your {0}",
    "link": "connection settings"
  },
  "fr": {
    "title": "Oops ... la connexion a été interrompue",
    "description": "Contrôle au sol à robot... répondez !",
    "helper_line1": "Merci de vérifier que le backend soit correctement lancé",
    "helper_line2": "ainsi que vos {0}",
    "link": "paramètres de connexion"
  }
}
</i18n>
