<template>
  <generic-action v-model="state" class="action-boolean" :device="device" :mode="mode">
    <template #action>
      <v-select
        v-model="state.path"
        :items="files"
        item-title="name"
        item-value="path"
        hide-details
        density="compact"
        @update:model-value="onCommand"
      />
      <v-btn
        :icon="state.status <= 0 ? 'mdi-play' : 'mdi-pause'"
        variant="text"
        color="success"
        size="x-large"
        density="compact"
        @click="state.status <= 0 ? onCommand(1) : onCommand(0)"
      />
      <v-btn
        :disabled="state.status < 0"
        icon="mdi-stop"
        variant="text"
        color="success"
        size="x-large"
        density="compact"
        @click="onCommand(-1)"
      />
    </template>
  </generic-action>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { Actuator, Mp3PlayerFile, Mp3PlayerState } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const state = defineModel<Mp3PlayerState>({ required: true });
const props = defineProps<{
  mode: HardwareMode;
  device: Actuator;
  files: Mp3PlayerFile[];
}>();

const deviceStore = useDeviceStore();
const loading = ref<boolean>(false);

const onCommand = (command: string | number) => {
  if (props.mode === HardwareMode.REALTIME) {
    loading.value = true;
    deviceStore
      .mutate(props.device.id, command)
      .then((ack: SocketAck) => {
        if (ack.success) {
          state.value = ack.success as Mp3PlayerState;
        }
        return;
      })
      .finally(() => (loading.value = false))
      .catch(logError);
  }
};
</script>
