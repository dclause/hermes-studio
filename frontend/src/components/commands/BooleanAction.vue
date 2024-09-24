<template>
  <generic-action
    v-model="state"
    class="action-boolean"
    :device="device"
    :mode="mode"
    :variant="variant"
  >
    <template #action>
      <v-switch
        v-model="innerValue"
        color="primary"
        density="compact"
        :disabled="loading"
        hide-details
        inline
        inset
        :loading="loading"
      >
        <template #label>
          <div class="d-none d-sm-block">
            {{
              $t('command.pin', {
                pin: device.pin,
                state: device.state,
              })
            }}
          </div>
        </template>
      </v-switch>
    </template>
  </generic-action>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { CommandMode, HardwareMode, logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { Actuator } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const state = defineModel<boolean>({ required: true });
const props = withDefaults(
  defineProps<{
    mode?: HardwareMode;
    variant?: CommandMode;
    device: Actuator;
  }>(),
  {
    mode: HardwareMode.REALTIME,
    variant: CommandMode.FULL,
  },
);

const deviceStore = useDeviceStore();
const loading = ref<boolean>(false);

/**
 * Mutates the state with validation from the server.
 */
const innerValue = computed<boolean>({
  get() {
    return state.value;
  },
  set(value) {
    previousState = innerValue.value;
    if (props.mode === HardwareMode.REALTIME) {
      loading.value = true;
      deviceStore
        .mutate(props.device.id, value)
        .then((ack: SocketAck) => {
          if (ack.error) {
            state.value = previousState;
          }
          return null;
        })
        .finally(() => (loading.value = false))
        .catch(logError);
    }
    state.value = value;
  },
});
let previousState = innerValue.value;
</script>
