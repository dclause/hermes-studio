<template>
  <div class="d-flex flex-grow-1 align-center command">
    <div v-if="mode != HardwareMode.NONE && board && !board.connected" class="text-center">
      <em>{{ $t('connexion.disconnect') }}</em>
    </div>
    <slot v-else name="action">
      <div class="font-italic text-error-lighten-1 command command-unknown">
        {{ $t('command.none') }}
      </div>
    </slot>
  </div>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { Actuator } from '@/types/devices';

const props = defineProps<{
  mode: HardwareMode;
  device: Actuator;
}>();

// Get the associated board.
const board = useBoardStore().get(props.device.bid);
</script>
