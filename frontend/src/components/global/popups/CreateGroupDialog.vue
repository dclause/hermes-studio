<template>
  <confirm-dialog
    v-model="createPopup"
    :title="t('title')"
    :confirm="toBeCreatedGroup?.id ? $t('form.edit') : $t('form.create')"
    @confirm="onCreate"
    @cancel="onCancel"
  >
    <v-text-field
      v-if="toBeCreatedGroup"
      v-model="toBeCreatedGroup.name"
      label="Name"
      required
      :rules="[Rule.REQUIRED]"
    />
  </confirm-dialog>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Rule } from '@/composables/formComposables';
import { Entity } from '@/types/core';

const { t } = useI18n();

const emit = defineEmits(['confirm', 'cancel']);
const toBeCreatedGroup = defineModel<Entity<unknown> | null>({ required: true });
const createPopup = ref<boolean>(!!toBeCreatedGroup.value);

// Force open the popup when toBeDeleted is set.
watch(toBeCreatedGroup, (newValue, oldValue) => {
  if (!oldValue && newValue) {
    createPopup.value = true;
  }
});

const onCancel = () => {
  emit('cancel');
  createPopup.value = false;
  toBeCreatedGroup.value = null;
};
const onCreate = () => {
  emit('confirm');
  createPopup.value = false;
  toBeCreatedGroup.value = null;
};
</script>

<i18n>
{
  "en": {
    "title": "Create a new group"
  },
  "fr": {
    "title": "Création d'un nouveau groupe"
  }
}
</i18n>
