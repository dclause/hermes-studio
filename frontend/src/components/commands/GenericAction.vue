<template>
  <div class="d-flex flex-grow-1 align-center action">
    <div v-if="mode != HardwareMode.OFF && board && !board.connected" class="text-center">
      <em>{{ $t('connexion.disconnect') }}</em>
    </div>
    <slot v-else name="action" v-bind="{ isCommandable: isCommandable }">
      <div class="font-italic text-error-lighten-1 action action-unknown">
        {{ $t('command.none') }}
      </div>
    </slot>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { Actuator } from '@/types/devices';

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
const isCommandable = computed(
  () => props.variant === CommandMode.FULL || props.variant === CommandMode.COMMAND,
);

// Get the associated board.
const board = computed(() => useBoardStore().get(props.device.bid));
</script>
