<template>
  <div v-if="board">
    <div class="d-flex justify-space-between align-center">
      <h1 class="text-h5 text-md-h4 d-flex align-center flex-grow-1">
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
      <v-btn color="primary" class="mr-5" :to="{ name: 'device.new', query: { board: board.id } }">
        <v-icon>mdi-plus</v-icon>
        <span class="d-none d-md-block ml-2">{{ t('new_device') }}</span>
      </v-btn>
      <v-btn
        color="primary"
        variant="tonal"
        class="d-none d-md-flex"
        :to="{ name: 'expander.new' }"
      >
        <v-icon>mdi-plus</v-icon>
        <span class="d-none d-md-block ml-2">{{ t('new_expander') }}</span>
      </v-btn>
    </div>
    <div class="ml-2 text-overline">
      <board-model style="line-height: 1em" class="mt-5" :model="board.model" />
      <protocol style="line-height: 1em" class="d-inline-block" :protocol="board.protocol" />
    </div>

    <v-tabs v-model="tab" bg-color="transparent" color="black" slider-color="primary">
      <!--      <v-tab value="info">-->
      <!--        {{ t('tab.info') }}-->
      <!--      </v-tab>-->
      <v-tab value="expanders">
        {{ t('tab.expanders') }}
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

      <v-tabs-window-item value="expanders">
        <div v-if="expanders.length">
          <component
            :is="useExpanderComponent(expander.type)"
            v-for="expander in expanders"
            :key="expander.id"
            :expander="expander"
            class="ml-2"
            @delete="onRequestDelete"
          />
        </div>
        <v-card-text v-else class="pa-8 text-center">
          <em>{{ t('no_expanders') }}</em>
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
import type { NestedGroup } from '@/types/groups';
import type { BoardId, Expander } from '@/types/hardwares';
import { storeToRefs } from 'pinia';
import { computed, ComputedRef, ref } from 'vue';
import { useI18n } from 'vue-i18n'; // Retrieve the board.
import { useRoute } from 'vue-router';
import { useExpanderComponent } from '@/composables/expanderComposables';
import { useFlatToNested } from '@/composables/groupComposables';
import { HardwareType } from '@/composables/hardwareComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { useExpanderStore } from '@/stores/expanderStore';
import { useGroupStore } from '@/stores/groupStore';
import { Device } from '@/types/devices';

const { t } = useI18n();

// Retrieve the board.
const route = useRoute();
const boardStore = useBoardStore();
const board = computed(() => boardStore.get(Number(route.params.hid) as BoardId));

// Retrieve the associated expanders.
const expanderStore = useExpanderStore();
const expanders = computed(() => expanderStore.list_by_board(board.value.id));

// Retrieve the associated devices.
const deviceStore = useDeviceStore();
const devices: ComputedRef<Device[]> = computed(() => {
  return [
    ...deviceStore.list_by_board(board.value.id),
    ...expanders.value.flatMap((expander) => deviceStore.list_by_expander(expander.id)),
  ];
});

const groupStore = useGroupStore();
const { groups } = storeToRefs(groupStore);
const shouldDisplayGroup = (group: NestedGroup): boolean => {
  // A group that have device should be displayed.
  if (group.device && devices.value.find((device) => device.id == group.device)) {
    return true;
  }
  // A group is display if at least one of the nested group must be displayed.
  return group.children.reduce((display, child) => display || shouldDisplayGroup(child), false);
};
const nestedGroups = computed(() => {
  return useFlatToNested(groups.value).filter((group) => shouldDisplayGroup(group));
});

// Selected tab.
const tab = ref('controls');

// Delete a group / device / expander.
const toBeDeleted = ref<Device | Expander | null>(null);
const onRequestDelete = (item: Device | Expander, type: HardwareType) => {
  toBeDeleted.value = { ...item, store: type };
};
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    switch (toBeDeleted.value.store) {
      case HardwareType.Device:
        deviceStore.delete((toBeDeleted.value as Device).id);
        break;
      case HardwareType.Expander:
        expanderStore.delete((toBeDeleted.value as Expander).id);
        break;
    }
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
    "new_expander": "New expander",
    "new_device": "New device",
    "type": "Board type: ",
    "status": "Status: ",
    "protocol": "Communication protocol: ",
    "tab": {
      "info": "Information",
      "expanders": "Expanders",
      "controls": "Controls and Actions",
      "inputs": "Sensors & Inputs",
      "history": "History"
    },
    "no_expanders": "No expander available for this board.",
    "no_actions": "No actions available for this board.",
    "no_inputs": "No inputs available for this board."
  },
  "fr": {
    "new_expander": "Nouvel extenseur",
    "new_device": "Nouveau device",
    "type": "Type de carte : ",
    "status": "Status : ",
    "protocol": "Protocol de communication : ",
    "tab": {
      "info": "Informations",
      "expanders": "Extenseurs",
      "controls": "Contrôles et Actions",
      "inputs": "Entrées et Capteurs",
      "history": "Historique"
    },
    "no_expanders": "Aucun extenseur disponible pour cette carte.",
    "no_actions": "Aucune contrôle disponible pour cette carte.",
    "no_inputs": "Aucun capteur disponible pour cette carte."
  }
}
</i18n>
