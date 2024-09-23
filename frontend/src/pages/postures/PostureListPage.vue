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
      <v-icon>mdi-plus</v-icon>
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

  <v-data-table
    v-model:items="items"
    class="posture-list"
    fixed-header
    :headers="headers"
    :loading="loading"
  >
    <template #loading>
      <v-skeleton-loader type="table-row@10" />
    </template>
    <template #headers="{ columns, isSorted, getSortIcon, toggleSort }">
      <tr>
        <template v-for="column in columns" :key="column.key">
          <th :class="`col-${column.key} ${column.headerProps?.class}`">
            <span class="mr-2 cursor-pointer" @click="() => toggleSort(column)">{{
              t(`headers.${column.title}`)
            }}</span>
            <template v-if="isSorted(column)">
              <v-icon :icon="getSortIcon(column)" />
            </template>
          </th>
        </template>
      </tr>
    </template>

    <template #[`item.play`]="{ item }">
      <v-btn
        icon="mdi-play"
        size="small"
        variant="outlined"
        :loading="loading"
        :disabled="loading || mode == HardwareMode.OFF"
        color="primary"
        @click="postureStore.play(item)"
      />
    </template>

    <template #[`item.name`]="{ item }">
      <app-link :to="{ name: 'posture.edit', params: { id: item.id } }">
        {{ item.name }}
      </app-link>
      <div class="font-italic">
        {{ item.description }}
      </div>
    </template>

    <template #[`item.actions`]="{ item }">
      <v-btn
        icon="mdi-pencil"
        size="small"
        :to="{ name: 'posture.edit', params: { id: item.id } }"
        variant="text"
      />
      <v-btn icon="mdi-trash-can" size="small" variant="text" @click="toBeDeleted = item" />
    </template>

    <template #no-data>
      <em>{{ t('empty') }}</em>
    </template>
  </v-data-table>

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { HardwareMode } from '@/composables/globalComposables';
import router from '@/plugins/router';
import { useBoardStore } from '@/stores/boardStore';
import { useConfigStore } from '@/stores/configurationStore';
import { usePostureStore } from '@/stores/postureStore';
import { Posture } from '@/types/postures';

const { t } = useI18n();
const { mode } = storeToRefs(useConfigStore());

const postureStore = usePostureStore();
const boardStore = useBoardStore();

// Selected tab.
const tab = ref('postures');

const { postures, loading } = storeToRefs(postureStore);
const items = computed<Posture[]>(() => Object.values(postures.value));

// Delete a posture.
const toBeDeleted = ref<Posture | null>(null);
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    postureStore.delete(toBeDeleted.value.id);
  }
};

// Posture list headers and data.
const headers = [
  { title: 'play', key: 'play' },
  { title: 'name', key: 'name', headerProps: { class: 'font-weight-bold' } },
  {
    title: 'actions',
    key: 'actions',
    headerProps: { class: 'text-center d-sm-table-cell font-weight-bold' },
    cellProps: { class: 'text-center' },
  },
];
</script>

<style lang="scss" scoped>
.posture-list {
  .col-play {
    width: 30px;
    text-align: center;
  }

  .col-actions {
    width: 120px;
  }
}
</style>

<i18n>
{
  "en": {
    "controls": "Robot control",
    "postures": "Postures",
    "new": "New posture",
    "empty": "No posture configured yet.",
    "headers": {
      "play": "",
      "name": "Name",
      "actions": "Actions"
    }
  },
  "fr": {
    "controls": "Contrôle du robot",
    "postures": "Postures",
    "new": "Nouvelle posture",
    "empty": "Aucune posture configurée pour le moment.",
    "headers": {
      "play": "",
      "name": "Nom",
      "actions": "Actions"
    }
  }
}
</i18n>
