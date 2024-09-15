<template>
  <div v-if="board">
    <div class="d-flex justify-space-between align-center">
      <h1 class="text-h5 text-md-h4 d-flex align-center">
        <board-connection-switch v-model="board" class="d-inline-block pr-3" />
        {{ board.name }}

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
              :disabled="!board.connected"
              @click="boardStore.reset(board.id)"
            />
          </template>
          <span>{{ $t('form.reset') }}</span>
        </v-tooltip>
      </h1>
      <v-btn
        color="primary"
        :icon="$vuetify.display.xs === true"
        :to="{ name: 'device.new', query: { board: board.id } }"
      >
        <v-icon>mdi-plus</v-icon>
        <span class="d-none d-sm-block">{{ t('new_device') }}</span>
      </v-btn>
    </div>
    <div class="ml-2 text-overline">
      <board-model :model="board.model" />
    </div>

    <v-tabs v-model="tab" bg-color="transparent" color="black" slider-color="primary">
      <v-tab value="info">
        {{ t('tab.info') }}
      </v-tab>
      <v-tab value="controls">
        {{ t('tab.controls') }}
      </v-tab>
      <v-tab value="inputs">
        {{ t('tab.inputs') }}
      </v-tab>
      <v-tab value="history">
        {{ t('tab.history') }}
      </v-tab>
    </v-tabs>

    <v-tabs-window v-model="tab">
      <v-tabs-window-item value="info">
        <v-card-text>
          <div>
            <span class="font-weight-bold">{{ t('type') }}</span>
            <board-model class="d-inline-block" :model="board.model" />
          </div>
          <div>
            <span class="font-weight-bold">{{ t('status') }}</span>
            {{ board.connected ? t('connexion.check') : $t('connexion.disconnect') }}
          </div>
          <div>
            <span class="font-weight-bold">{{ t('protocol') }}</span>
            <protocol class="d-inline-block" :protocol="board.protocol" />
          </div>
        </v-card-text>
      </v-tabs-window-item>

      <v-tabs-window-item value="controls">
        <div v-if="nestedGroups.length">
          <nested-group v-model="nestedGroups" @delete="onRequestDelete" />
        </div>
        <v-card-text v-else class="pa-8 text-center">
          <em>{{ t('no_actions') }}</em>
        </v-card-text>
      </v-tabs-window-item>

      <v-tabs-window-item value="inputs">
        <v-card-text class="pa-8 text-center">
          <em>{{ t('no_inputs') }}</em>
        </v-card-text>
      </v-tabs-window-item>

      <v-tabs-window-item value="history">
        <v-card-text class="pa-8 text-center">
          @todo
        </v-card-text>
      </v-tabs-window-item>
    </v-tabs-window>

    <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
  </div>
</template>
<script lang="ts" setup>
import type { BoardId } from '@/types/boards';
import type { Device } from '@/types/devices';
import type { FlatGroup, GroupId } from '@/types/groups';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n'; // Retrieve the board.
import { useRoute } from 'vue-router';
import { useFlatToNested } from '@/composables/groupComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { useGroupStore } from '@/stores/groupStore';

const { t } = useI18n();

// Retrieve the board.
const route = useRoute();
const boardStore = useBoardStore();
const board = computed(() => boardStore.get(Number(route.params.bid) as BoardId));

// Retrieve the associated devices.
const deviceStore = useDeviceStore();
const devices = computed(() => deviceStore.list_by_board(board.value.id));

const groupStore = useGroupStore();
const { groups } = storeToRefs(groupStore);
const nestedGroups = computed(() => {
  const filteredGroups = Object.values(groups.value).reduce(
    (acc, group) => {
      if (!group.device || devices.value.find((device) => device.id == group.device)) {
        acc[group.id] = group;
      }
      return acc;
    },
    {} as Record<GroupId, FlatGroup>,
  );
  return useFlatToNested(filteredGroups);
});

// Selected tab.
const tab = ref('controls');

// Delete a group / device.
const toBeDeleted = ref<Device | null>(null);
const onRequestDelete = (item: Device) => {
  toBeDeleted.value = item;
};
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    deviceStore.delete(toBeDeleted.value.id);
  }
};
</script>

<style lang="scss" scoped>
.wrapper {
  overflow: visible;
}
</style>

<i18n>
{
  "en": {
    "new_device": "New device",
    "type": "Board type: ",
    "status": "Status: ",
    "protocol": "Communication protocol: ",
    "tab": {
      "info": "Information",
      "history": "History",
      "controls": "Controls and Actions",
      "inputs": "Sensors & Inputs"
    },
    "no_actions": "No actions available for this board.",
    "no_inputs": "No inputs available for this board."
  },
  "fr": {
    "new_device": "Nouveau device",
    "type": "Type de carte : ",
    "status": "Status : ",
    "protocol": "Protocol de communication : ",
    "tab": {
      "info": "Informations",
      "history": "Historique",
      "controls": "Contrôles et Actions",
      "inputs": "Entrées et Capteurs"
    },
    "no_actions": "Aucune contrôle disponible pour cette carte.",
    "no_inputs": "Aucun capteur disponible pour cette carte."
  }
}
</i18n>
