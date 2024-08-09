<template>
  <div v-if="board">
    <div class="d-flex justify-space-between align-center">
      <h1 class="text-h5 text-md-h4 d-flex align-center">
        <board-connection-switch v-model="board" class="d-inline-block pr-3" />
        {{ board.name }}
      </h1>
      <v-btn
        color="primary"
        :icon="$vuetify.display.xs === true"
        :to="{ name: 'device.new', params: { bid: board.id } }"
      >
        <v-icon>mdi-plus</v-icon>
        <span class="d-none d-sm-block">{{ t('new_device') }}</span>
      </v-btn>
    </div>
    <div class="ml-2 text-overline">
      <board-model :model="board.model" />
    </div>

    <v-tabs v-model="tab" background-color="transparent" color="black" slider-color="primary">
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

    <v-window v-model="tab">
      <v-window-item value="info">
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
      </v-window-item>

      <v-window-item value="controls">
        <v-card-text v-if="actuators.length">
          <div
            v-for="(device, id) in actuators"
            :key="device.id"
            class="d-flex flex-1-1-100 align-center"
          >
            <actuator v-model="actuators[id]" />
            <v-btn
              icon="mdi-pencil"
              size="small"
              :to="{
                name: 'device.edit',
                params: { id: device.id },
              }"
              variant="text"
            />
            <v-btn
              icon="mdi-trash-can"
              size="small"
              variant="text"
              @click="openConfirmDeletePopup(device)"
            />
          </div>
        </v-card-text>
        <v-card-text v-else class="pa-8 text-center">
          <em>{{ t('no_actions') }}</em>
        </v-card-text>
      </v-window-item>

      <v-window-item value="inputs">
        <v-card-text class="pa-8 text-center">
          <em>{{ t('no_inputs') }}</em>
        </v-card-text>
      </v-window-item>

      <v-window-item value="history">
        <v-card-text class="pa-8 text-center">
          @todo
        </v-card-text>
      </v-window-item>
    </v-window>

    <v-dialog v-model="confirmPopup" width="auto">
      <v-card>
        <v-card-text> Are you sure to delete the device '{{ selectedDevice?.name }}'</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="green-darken-1" variant="text" @click="cancelDelete">
            {{ t('form.cancel') }}
          </v-btn>
          <v-btn color="green-darken-1" variant="text" @click="confirmDelete()">
            {{ t('form.delete') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>
<script lang="ts" setup>
import type { BoardId } from '@/types/boards';
import type { Actuator } from '@/types/devices';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n'; // Retrieve the board.
import { useRoute } from 'vue-router'; // import { useDeviceComponent } from '@/composables/hardware';
import { useDeviceStore } from '@/stores/actuatorStore';
import { useBoardStore } from '@/stores/boardStore';

const { t } = useI18n();

// Retrieve the board.
const route = useRoute();
const boardStore = useBoardStore();
const board = computed(() => boardStore.get(Number(route.params.bid) as BoardId));

// Retrieve the associated devices.
const deviceStore = useDeviceStore();
const actuators = computed(() => deviceStore.list_by_board(board.value.id));

// const deviceComponents = Object.values(board.devices).reduce<Record<string, unknown>>(
//   (acc, device) => {
//     acc[device.config.type] = useDeviceComponent(device.config.type);
//     return acc;
//   },
//   {},
// );
const tab = ref('controls');

// Delete device.
const confirmPopup = ref<boolean>(false);
const selectedDevice = ref<Actuator | null>(null);
const openConfirmDeletePopup = (item: Actuator) => {
  selectedDevice.value = item;
  confirmPopup.value = true;
};
const cancelDelete = () => {
  selectedDevice.value = null;
  confirmPopup.value = false;
};
const confirmDelete = () => {
  // if (toBeDelete.value) {
  //   // boardStore.boards.delete(toBeDelete.value.id).catch(logError);
  // }
  cancelDelete();
};
</script>

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
