<template>
  <v-dialog v-model="openDialog" max-width="400" persistent>
    <v-card prepend-icon="mdi-alert-outline" :title="title ?? t('title')">
      <v-card-text>
        <slot>{{ text ?? t('text') }}</slot>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="green-darken-1" variant="text" @click="cancelAction">
          {{ cancel ?? $t('form.cancel') }}
        </v-btn>
        <v-btn color="green-darken-1" variant="text" @click="confirmAction">
          {{ confirm ?? $t('form.confirm') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const emit = defineEmits(['confirm', 'cancel']);
const openDialog = defineModel<boolean>({ required: true });
defineProps<{
  title?: string;
  text?: string;
  cancel?: string;
  confirm?: string;
}>();

const cancelAction = () => {
  emit('cancel');
  openDialog.value = false;
};
const confirmAction = () => {
  emit('confirm');
  openDialog.value = false;
};
</script>

<i18n>
{
  "en": {
    "title": "Confirmation required",
    "text": "This action cannot be undone. Please confirm."
  },
  "fr": {
    "title": "Confirmation requise",
    "text": "Cette action est irréversible. Veuillez confirmer."
  }
}
</i18n>
