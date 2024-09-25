<template>
  <v-card
    class="wrapper flex-1-1-100 align-center mt-2 overflow-visible"
    :class="{
      'd-flex': !isChip,
      'd-inline-flex': isChip,
      [variant]: true,
    }"
    :variant="cardVariant"
  >
    <slot name="prefix" v-bind="{ isEditable: isEditable, isChip: isChip }" />

    <div class="d-flex flex-1-1-100 align-center mt-2 mb-2 command">
      <div class="d-none d-sm-block">
        <slot name="icon">
          <v-icon class="mx-2" icon="mdi-progress-question" size="30" />
        </slot>
      </div>

      <v-label class="command-label ml-2">
        <slot name="label">
          <div class="font-weight-bold">
            {{ device.name }}
          </div>
          <div class="text-body-2 font-italic">
            {{ board.name }}
          </div>
        </slot>
      </v-label>

      <div
        v-if="!isChip"
        class="command-pin ml-2 mr-2 text-lowercase font-italic d-none d-sm-block"
      >
        <slot name="info">
          {{ $t('command.pin') }}: {{ device.pin ?? '??' }}
        </slot>
      </div>

      <slot v-if="!isChip" name="command" class="flex-grow-1">
        <div class="font-italic text-error-lighten-1">
          {{ $t('command.none') }}
        </div>
      </slot>
    </div>

    <div v-if="!isChip" class="d-flex">
      <v-btn
        icon="mdi-refresh"
        size="small"
        variant="text"
        :disabled="!board.connected"
        @click="onReset(device)"
      />

      <v-btn
        v-if="isEditable"
        icon="mdi-pencil"
        size="small"
        :to="{
          name: 'device.edit',
          params: { id: device.id },
        }"
        variant="text"
      />

      <v-btn
        v-if="isEditable"
        icon="mdi-trash-can"
        size="small"
        variant="text"
        @click="emit('delete', device)"
      />
    </div>
  </v-card>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { CommandMode } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { Actuator, Device, DeviceState } from '@/types/devices';

const emit = defineEmits<{ delete: [item: Actuator]; reset: [value: DeviceState] }>();
const props = withDefaults(
  defineProps<{
    device: Actuator;
    variant?: CommandMode;
  }>(),
  { variant: CommandMode.FULL },
);
const isChip = computed(() => props.variant === CommandMode.NONE);
const isEditable = computed(() => props.variant === CommandMode.FULL);

const boardStore = useBoardStore();

const board = computed(() => boardStore.get(props.device.bid));

const cardVariant = computed(() => {
  switch (props.variant) {
    case CommandMode.COMMAND:
    case CommandMode.KEYFRAME:
      return 'flat';
    case CommandMode.FULL:
    default:
      return 'elevated';
  }
});

// Reset button
const deviceStore = useDeviceStore();
const onReset = (device: Device) => {
  deviceStore.reset(device.id);
  emit('reset', device.default);
};
</script>

<style lang="scss" scoped>
.command-label {
  flex: 1 1 100%;
  text-overflow: ellipsis;
  display: block;
  @media (min-width: 460px) {
    width: 10rem;
    flex: none;
  }
}

.chip {
  padding-right: 1em;

  .command-label {
    width: auto;
  }
}

.command-pin {
  width: 4rem;
}
</style>
