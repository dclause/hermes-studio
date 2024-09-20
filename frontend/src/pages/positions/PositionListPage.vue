<template>
  <div class="d-flex align-center mb-4">
    <h1 class="text-h5 text-md-h4 flex-grow-1">
      <v-icon icon="mdi-camera-control" />
      {{ t('title') }}

      <v-tooltip location="bottom">
        <template #activator="{ props }">
          <v-btn
            v-bind="props"
            size="large"
            rounded="xl"
            class="ml-4 pa-0"
            variant="text"
            icon="mdi-refresh"
            density="comfortable"
            @click="boardStore.reset_all()"
          />
        </template>
        <span>{{ $t('form.reset') }}</span>
      </v-tooltip>
    </h1>
  </div>

  <v-spacer class="my-3" />

  <nested-group v-model="nestedGroups" :disabled="loading" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useFlatToNested } from '@/composables/groupComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useGroupStore } from '@/stores/groupStore';

const { t } = useI18n();
const groupStore = useGroupStore();
const boardStore = useBoardStore();

const { groups, loading } = storeToRefs(groupStore);
const nestedGroups = computed(() => {
  return useFlatToNested(groups.value);
});
</script>

<i18n>
{
  "en": {
    "title": "Robot control",
    "empty": "No controls configured yet."
  },
  "fr": {
    "title": "Contrôle du robot",
    "empty": "Aucune contrôle configurée pour le moment."
  }
}
</i18n>
