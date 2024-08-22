<template>
  <confirm-dialog
    v-model="createPopup"
    :icon="group?.id ? 'mdi-pencil' : 'mdi-alert-outline'"
    :title="group?.id ? t('title.edit') : t('title.create')"
    :confirm="group?.id ? $t('form.edit') : $t('form.create')"
    @confirm="onCreate"
    @cancel="onCancel"
  >
    <v-text-field
      v-if="group"
      v-model="group.name"
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
const group = defineModel<Entity<unknown> | null>({ required: true });
const createPopup = ref<boolean>(!!group.value);

// Force open the popup when toBeDeleted is set.
watch(group, (newValue, oldValue) => {
  if (!oldValue && newValue) {
    createPopup.value = true;
  }
});

const onCancel = async () => {
  createPopup.value = false;
  emit('cancel');
  setTimeout(() => {
    group.value = null;
  }, 500);
};
const onCreate = () => {
  emit('confirm');
  createPopup.value = false;
  setTimeout(() => {
    group.value = null;
  }, 500);
};
</script>

<i18n>
{
  "en": {
    "title": {
      "create": "Create a new group",
      "edit": "Edit the group"
    }
  },
  "fr": {
    "title": {
      "create": "Création d'un nouveau groupe",
      "edit": "Editer le groupe"
    }
  }
}
</i18n>
