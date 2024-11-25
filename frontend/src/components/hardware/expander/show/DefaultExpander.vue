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

    <div class="d-flex flex-1-1-100 align-center mt-2 mb-2 expander">
      <div class="d-none d-sm-block">
        <slot name="icon">
          <v-icon class="mx-2" icon="mdi-progress-question" size="30" />
        </slot>
      </div>

      <v-label class="expander-label ml-2">
        <slot name="label">
          <div class="font-weight-bold">
            {{ expander.name }}
          </div>
          <div class="text-body-2 font-italic">
            {{ board.name }}
          </div>
        </slot>
      </v-label>

      <slot v-if="!isChip" name="command">
        <div
          v-if="mode != HardwareMode.OFF && board && !board.connected"
          class="text-center font-italic text-error-lighten-1 expander-command"
        >
          {{ $t('connexion.disconnect') }}
        </div>
      </slot>

      <div v-if="!isChip" class="expander-info ml-2 mr-2 font-italic d-none d-sm-block flex-grow-1">
        <slot name="info" />
      </div>
    </div>

    <div class="d-flex">
      <v-btn
        v-if="isEditable"
        icon="mdi-pencil"
        size="small"
        :to="{
          name: 'expander.edit',
          params: { id: expander.id },
        }"
        variant="text"
      />

      <v-btn
        v-if="isEditable"
        icon="mdi-trash-can"
        size="small"
        variant="text"
        @click="emit('delete', expander, HardwareType.Expander)"
      />
    </div>
  </v-card>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { HardwareType } from '@/composables/hardwareComposables';
import { useBoardStore } from '@/stores/boardStore';
import { BoardId, Expander } from '@/types/hardwares';

const emit = defineEmits<{ delete: [item: Expander, type: HardwareType] }>();
const props = withDefaults(
  defineProps<{
    expander: Expander;
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  { mode: HardwareMode.REALTIME, variant: CommandMode.FULL },
);

const isChip = computed(() => props.variant === CommandMode.NONE);
const isEditable = computed(() => props.variant === CommandMode.FULL);

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

const boardStore = useBoardStore();
const board = computed(() => boardStore.get(props.expander.hid.id as BoardId));
</script>

<style lang="scss" scoped>
.expander-label {
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

  .expander-label {
    width: auto;
  }
}

.expander-command {
  width: 8rem;
}
</style>
