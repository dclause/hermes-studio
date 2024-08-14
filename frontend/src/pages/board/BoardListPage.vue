<template>
  <div class="d-flex justify-space-between align-center mb-4">
    <h1 class="text-h5 text-md-h4">
      <v-icon icon="mdi-cog-transfer" />
      {{ t('title') }}
    </h1>
    <v-btn color="primary" :icon="$vuetify.display.xs === true" :to="{ name: 'board.new' }">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-sm-block">{{ t('new') }}</span>
    </v-btn>
  </div>

  <v-btn
    append-icon="mdi-refresh"
    class="pa-2 pt-0 pb-0"
    :disabled="loading"
    text="Refresh"
    variant="text"
    @click="boardStore.refresh()"
  />

  <v-spacer />

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

    <!--    <template #bottom="slots">-->
    <!--      <v-row class="mt-1">-->
    <!--        &lt;!&ndash;        <div class="align-self-center text-center text-md-left flex-1-1"></div>&ndash;&gt;-->
    <!--        <v-data-table-footer class="flex-1-1"></v-data-table-footer>-->
    <!--      </v-row>-->
    <!--    </template>-->
  </v-data-table>

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onDelete" />
</template>

<script lang="ts" setup>
import type { Board } from '@/types/boards';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useBoardStore } from '@/stores/boardStore';

const { t } = useI18n();

const boardStore = useBoardStore();
const { loading, boards } = storeToRefs(boardStore);
const items = computed<Board[]>(() => Object.values(boards.value));

// Delete a board.
const toBeDeleted = ref<Board | null>(null);
const onDelete = () => {
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
}
</style>

<style lang="scss" scoped>
.board-list {
  .col-icon,
  .col-status {
    width: 30px;
    text-align: center;
  }
}
</style>

<i18n>
{
  "en": {
    "title": "My cards",
    "headers": {
      "status": "Status",
      "name": "Name",
      "type": "Board type",
      "protocol": "Communication protocol",
      "actions": "Actions"
    },
    "new": "Add a card",
    "empty": "No card configured yet."
  },
  "fr": {
    "title": "Mes cartes",
    "headers": {
      "status": "Status",
      "name": "Nom",
      "type": "Type de carte",
      "protocol": "Protocol de communication",
      "actions": "Actions"
    },
    "new": "Ajouter une carte",
    "empty": "Aucune carte configurée pour le moment."
  }
}
</i18n>
