<template>
  <generic-action v-model="state" class="action-boolean" :device="device" :mode="mode">
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
import { computed, MaybeRef, ref } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { Actuator, DeviceState } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const state = defineModel<DeviceState>({ required: true });
const props = defineProps<{
  mode: HardwareMode;
  device: Actuator;
  true: DeviceState;
  false: DeviceState;
}>();

const deviceStore = useDeviceStore();
const loading = ref<boolean>(false);

/**
 * Mutates the state with validation from the server.
 * If the mutation goes wrong: returns the state value to previous.
 * @todo: remove previous value when set_state accepts any state
 */
const innerValue = computed<boolean>({
  get() {
    return state.value === props.true;
  },
  set(value) {
    previousValue = innerValue.value;
    if (props.mode === HardwareMode.REALTIME) {
      loading.value = true;
      deviceStore
        .mutate(props.device.id, getState(value))
        .then((ack: SocketAck) => {
          if (ack.error) {
            state.value = getState(previousValue);
          }
          return null;
        })
        .finally(() => {
          loading.value = false;
        })
        .catch(logError);
    }
    state.value = getState(value);
  },
});
let previousValue = innerValue.value;

const getState = (value: MaybeRef<boolean>) => (value ? props['true'] : props['false']);
</script>
