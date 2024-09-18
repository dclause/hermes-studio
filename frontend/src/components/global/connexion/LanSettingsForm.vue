<template>
  <v-card class="mx-auto my-8" variant="flat">
    <v-card-item>
      <v-card-title>{{ t('title') }}</v-card-title>
      <v-card-subtitle>
        {{ t('description') }}
      </v-card-subtitle>
    </v-card-item>

    <v-card-text>
      <v-form v-model="form" :disabled="disabled" :loading="loading" @submit.prevent="onSubmit">
        <v-row>
          <v-col>
            <v-text-field
              v-model="socket.url"
              :disabled="socket.isConnected"
              :rules="[Rule.REQUIRED, Rule.NON_EMPTY]"
              :hint="t('hint')"
              required
              type="url"
            />
          </v-col>
          <v-col>
            <v-btn
              v-if="socket.isConnected"
              class="mt-1"
              color="primary"
              density="default"
              size="large"
              type="submit"
            >
              {{ t('disconnect') }}
            </v-btn>
            <v-btn
              v-else
              class="mt-1"
              color="success"
              density="default"
              size="large"
              type="submit"
            >
              {{ t('connect') }}
            </v-btn>
          </v-col>
        </v-row>
      </v-form>
    </v-card-text>
  </v-card>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { Rule } from '@/composables/formComposables';
import { useConnectionStore } from '@/stores/connectionStore';
import { useToasterStore } from '@/stores/toastStore';

const { t } = useI18n();
const socket = useConnectionStore();

const form = defineModel<boolean>();
const disabled = ref<boolean>(false);
const loading = ref<boolean>(false);
const toaster = useToasterStore();

// const onValidate = (value) => !!value && !!data.value && !error.value && 'URL is incorrect';
const onSubmit = async () => {
  disabled.value = loading.value = true;
  if (socket.isConnected) {
    socket.close();
  } else {
    if ((await fetch(`${socket.url}/api`)).ok) {
      socket.open();
    } else {
      toaster.error('Server connection failed.');
    }
  }
  loading.value = disabled.value = false;
};
</script>

<i18n>
{
  "en": {
    "connect": "Connect",
    "disconnect": "Disconnect",
    "title": "Connection settings",
    "description": "Configure connection settings to Hermes-Studio backend.",
    "hint": "The URL given by hermes-studio executable when started. By default: localhost:4000"
  },
  "fr": {
    "connect": "Connecter",
    "disconnect": "Déconnecter",
    "title": "Paramètres de connexion",
    "description": "Configurer les informations de connection vers le backend Hermes-Studio.",
    "hint": "L'URL donnée par l'application hermes-studio au démarrage. Par défaut: localhost:4000"
  }
}
</i18n>
