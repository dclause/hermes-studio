<template>
  <confirm-dialog
    v-model="confirmPopup"
    :title="t('title')"
    :text="t('text', { name: toBeDeleted?.name })"
    :confirm="$t('form.delete')"
    @confirm="onDelete"
    @cancel="onCancel"
  />
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Entity } from '@/types/core';

const { t } = useI18n();

const emit = defineEmits(['confirm', 'cancel']);
const toBeDeleted = defineModel<Entity<unknown> | null>({ required: true });
const confirmPopup = ref<boolean>(!!toBeDeleted.value);

// Force open the popup when toBeDeleted is set.
watch(toBeDeleted, (newValue, oldValue) => {
  if (!oldValue && newValue) {
    confirmPopup.value = true;
  }
});

const onCancel = () => {
  emit('cancel');
  confirmPopup.value = false;
  toBeDeleted.value = null;
};
const onDelete = () => {
  emit('confirm');
  confirmPopup.value = false;
  toBeDeleted.value = null;
};
</script>

<i18n>
{
  "en": {
    "title": "This action cannot be undone",
    "text": "Are you sure to delete '{name}' ?"
  },
  "fr": {
    "title": "Cette action est irréversible",
    "text": "Êtes-vous certain de supprimer '{name}' ?"
  }
}
</i18n>
