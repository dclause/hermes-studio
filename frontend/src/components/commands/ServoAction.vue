<template>
  <generic-action v-model="state" class="action-servo" :device="device" :mode="mode">
    <template #action>
      <v-slider
        v-model="innerValue"
        class="action-slider d-none ma-0"
        color="primary"
        :disabled="loading"
        :loading="loading"
        :min="min"
        :max="max"
        :step="1"
        hide-details
        thumb-size="20px"
        track-color="grey"
        track-size="8px"
        :thumb-label="true"
        @end="onSlider"
      >
        <template #prepend>
          <v-btn icon="mdi-minus" size="small" variant="text" @click="onDecrement" />
        </template>

        <template #append>
          <v-btn icon="mdi-plus" size="small" variant="text" @click="onIncrement" />
        </template>

        <template #thumb-label="{ modelValue }">
          <div>{{ modelValue }}°</div>
        </template>
      </v-slider>
      <v-text-field
        v-model.number="state"
        class="action-input flex-0-0"
        density="compact"
        :disabled="loading"
        hide-details
        :loading="loading"
        :min="min"
        :max="max"
        single-line
        type="number"
        @change="onUserInput"
      />
    </template>
  </generic-action>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { Servo } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const state = defineModel<number>({ required: true });
const props = defineProps<{
  mode: HardwareMode;
  device: Servo;
  min: number;
  max: number;
}>();

const deviceStore = useDeviceStore();
const loading = ref<boolean>(false);

/**
 * Mutates the state with validation from the server.
 */
const innerValue = computed<number>({
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

const onSlider = (value: number) => {
  innerValue.value = value;
};
const onUserInput = (event: InputEvent) => {
  const value = Math.min(props.max as number, Number((event.target as HTMLInputElement).value));
  innerValue.value = Math.max(props.min as number, value);
};
const onIncrement = () => {
  innerValue.value = Math.min(props.max as number, ++(state.value as number));
};
const onDecrement = () => {
  innerValue.value = Math.max(props.min as number, --(state.value as number));
};
</script>

<style lang="scss" scoped>
.action-servo {
  .action-slider {
    @media (min-width: 850px) {
      display: grid !important;
    }
  }

  .action-input {
    width: 90px;

    input,
    .v-field__input {
      padding: 0 10px;
    }
  }
}
</style>
