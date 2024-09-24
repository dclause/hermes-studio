<template>
  <div class="d-flex align-center mb-4">
    <h1 class="text-h5 text-md-h4 flex-grow-1">
      <v-icon icon="mdi-camera-control" />
      {{ t('controls') }}

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
    <v-btn color="primary" :to="{ name: 'posture.new' }">
      <v-icon>mdi-content-save</v-icon>
      <span class="d-none d-md-block ml-2">{{ t('new') }}</span>
    </v-btn>
  </div>

  <v-tabs v-model="tab">
    <v-tab value="controls" @click="router.push({ name: 'posture.control' })">
      {{ t('controls') }}
    </v-tab>
    <v-tab value="postures" @click="router.push({ name: 'posture.list' })">
      {{ t('postures') }}
    </v-tab>
  </v-tabs>

  <v-spacer class="my-3" />

  <nested-group v-model="nestedGroups" variant="minimal" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useFlatToNested } from '@/composables/groupComposables';
import router from '@/plugins/router';
import { useBoardStore } from '@/stores/boardStore';
import { useGroupStore } from '@/stores/groupStore';

const { t } = useI18n();
const groupStore = useGroupStore();
const boardStore = useBoardStore();

// Selected tab.
const tab = ref('controls');

const { groups } = storeToRefs(groupStore);
const nestedGroups = computed(() => {
  return useFlatToNested(groups.value);
});
</script>

<i18n>
{
  "en": {
    "controls": "Robot control",
    "postures": "Postures",
    "new": "Save as posture",
    "empty": "No controls configured yet."
  },
  "fr": {
    "controls": "Contrôle du robot",
    "postures": "Postures",
    "new": "Sauvegarder la posture",
    "empty": "Aucune contrôle configurée pour le moment."
  }
}
</i18n>
