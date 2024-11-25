<template>
  <div class="d-flex flex-grow-1 align-center action">
    <slot
      v-if="mode == HardwareMode.OFF || (board && board.connected)"
      name="action"
      v-bind="{ isCommandable: isCommandable }"
    >
      <div class="font-italic text-error-lighten-1 action action-unknown">
        {{ $t('command.none') }}
      </div>
    </slot>
    <div v-else class="text-center">
      <em>{{ $t('connexion.disconnect') }}</em>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { useHardware } from '@/composables/hardwareComposables';
import { OutputDevice } from '@/types/devices';

const props = withDefaults(
  defineProps<{
    mode?: HardwareMode;
    variant?: CommandMode;
    device: OutputDevice;
  }>(),
  {
    mode: HardwareMode.REALTIME,
    variant: CommandMode.FULL,
  },
);
const isCommandable = computed(
  () => props.variant === CommandMode.FULL || props.variant === CommandMode.COMMAND,
);

// Get the associated board.
const { get_board } = useHardware();
const board = computed(() => get_board(props.device.hid));
</script>
