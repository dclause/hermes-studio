<template>
  <div class="d-flex justify-space-between align-center mb-4">
    <h1 class="text-h5 text-md-h4">
      <v-icon icon="mdi-cog-transfer" />
      {{ t('title') }}
    </h1>
    <v-btn color="primary" :to="{ name: 'board.new' }">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-md-block ml-2">{{ t('new') }}</span>
    </v-btn>
  </div>

  <v-tabs v-model="tab">
    <v-tab value="boards" @click="router.push({ name: 'board.list' })">
      {{ t('boards') }}
    </v-tab>
    <v-tab value="devices" @click="router.push({ name: 'device.list' })">
      {{ t('devices') }}
    </v-tab>
  </v-tabs>

  <v-spacer class="my-3" />

  <v-data-table
    v-model:items="items"
    class="board-list"
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

    <template #[`item.status`]="{ index }">
      <board-connection-switch v-model="items[index]" />
    </template>
    <template #[`item.name`]="{ item }">
      <app-link :to="{ name: 'board.show', params: { bid: item.id } }">
        {{ item.name }}
      </app-link>
    </template>
    <template #[`item.type`]="{ item }">
      <board-model :model="item.model" />
    </template>
    <template #[`item.protocol`]="{ item }">
      <protocol :protocol="item.protocol" />
    </template>

    <template #[`item.actions`]="{ item }">
      <v-btn
        icon="mdi-pencil"
        size="small"
        :to="{ name: 'board.edit', params: { bid: item.id } }"
        variant="text"
      />
      <v-btn
        ref="deleteBtn"
        icon="mdi-trash-can"
        size="small"
        variant="text"
        @click="toBeDeleted = item"
      />
    </template>

    <template #no-data>
      <em>{{ t('empty') }}</em>
    </template>
  </v-data-table>

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
</template>

<script lang="ts" setup>
import type { Board } from '@/types/boards';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import router from '@/plugins/router';
import { useBoardStore } from '@/stores/boardStore';

const { t } = useI18n();

const boardStore = useBoardStore();
const { loading, boards } = storeToRefs(boardStore);
const items = computed<Board[]>(() => Object.values(boards.value));

// Selected tab.
const tab = ref('boards');

// Delete a board.
const toBeDeleted = ref<Board | null>(null);
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    boardStore.delete(toBeDeleted.value.id);
  }
};

// Board list headers and data.
const headers = [
  {
    title: 'status',
    key: 'status',
    headerProps: { class: 'text-center d-sm-table-cell font-weight-bold' },
    cellProps: { class: 'text-center d-sm-table-cell' },
  },
  { title: 'name', key: 'name', headerProps: { class: 'font-weight-bold' } },
  {
    title: 'type',
    key: 'type',
    headerProps: { class: 'd-none d-md-table-cell font-weight-bold' },
    cellProps: { class: 'd-none d-md-table-cell' },
  },
  {
    title: 'protocol',
    key: 'protocol',
    headerProps: { class: 'd-none d-sm-table-cell font-weight-bold' },
    cellProps: { class: 'd-none d-sm-table-cell' },
  },
  {
    title: 'actions',
    key: 'actions',
    headerProps: { class: 'text-center font-weight-bold' },
    cellProps: { class: 'text-center' },
  },
];
</script>

<style lang="scss" scoped>
.board-list {
  .col-icon,
  .col-status {
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
    "title": "Manage boards",
    "boards": "Boards",
    "devices": "Groups & Devices",
    "headers": {
      "status": "Status",
      "name": "Name",
      "type": "Board type",
      "protocol": "Communication protocol",
      "actions": "Actions"
    },
    "new": "New Board",
    "empty": "No board configured yet."
  },
  "fr": {
    "title": "Configuration des cartes",
    "boards": "Cartes",
    "devices": "Groupes & Composants",
    "headers": {
      "status": "Status",
      "name": "Nom",
      "type": "Type de carte",
      "protocol": "Protocol de communication",
      "actions": "Actions"
    },
    "new": "Nouvelle Carte",
    "empty": "Aucune carte configurée pour le moment."
  }
}
</i18n>
