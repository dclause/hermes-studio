<template>
  <div class="d-flex justify-space-between align-center mb-4">
    <h1 class="text-h5 text-md-h4">
      <v-icon icon="mdi-movie-open" />
      {{ t('title') }}
    </h1>
    <v-btn color="primary" :icon="$vuetify.display.xs === true" :to="{ name: 'animation.new' }">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-sm-block">{{ t('new') }}</span>
    </v-btn>
  </div>

  <v-spacer />

  <v-data-table
    v-model:items="items"
    class="animation-list"
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
        v-if="!item.playing"
        icon="mdi-play"
        size="small"
        variant="outlined"
        :loading="loading"
        :disabled="loading || mode == HardwareMode.OFF"
        color="primary"
        @click="animationStore.play(item)"
      />
      <v-progress-circular
        v-else
        :model-value="(item.progress * 360) / item.duration"
        :indeterminate="!item.progress"
        :rotate="360"
        :size="40"
        :width="5"
        color="primary"
      >
        <template #default>
          <v-btn
            icon="mdi-stop"
            :disabled="loading || mode == HardwareMode.OFF"
            size="small"
            variant="text"
            color="primary"
            @click="animationStore.stop(item)"
          />
        </template>
      </v-progress-circular>
    </template>

    <template #[`item.name`]="{ item }">
      <app-link :to="{ name: 'animation.edit', params: { id: item.id } }">
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
        :to="{ name: 'animation.edit', params: { id: item.id } }"
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
import { useAnimationStore } from '@/stores/animationStore';
import { useConfigStore } from '@/stores/configurationStore';
import { Animation } from '@/types/animation';

const { t } = useI18n();
const { mode } = storeToRefs(useConfigStore());

// Get all animations.
const animationStore = useAnimationStore();
const { loading, animations } = storeToRefs(animationStore);
const items = computed<Animation[]>(() => Object.values(animations.value));

// Delete animation.
const toBeDeleted = ref<Animation | null>(null);
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    animationStore.delete(toBeDeleted.value.id);
  }
};

// Animation list headers and data.
const headers = [
  { title: 'play', key: 'play' },
  { title: 'name', key: 'name', headerProps: { class: '' } },
  {
    title: 'actions',
    key: 'actions',
    headerProps: { class: 'text-center' },
    cellProps: { class: 'text-center' },
  },
];
</script>

<style lang="scss" scoped>
.animation-list {
  .col-play {
    width: 30px;
    text-align: center;
  }
}
</style>

<i18n>
{
  "en": {
    "title": "Animations",
    "headers": {
      "play": "",
      "name": "Name",
      "actions": "Actions"
    },
    "new": "New animation",
    "empty": "No animations configured yet."
  },
  "fr": {
    "title": "Animations",
    "headers": {
      "play": "",
      "name": "Nom",
      "actions": "Actions"
    },
    "new": "Nouvelle animation",
    "empty": "Aucune animation pour le moment."
  }
}
</i18n>
