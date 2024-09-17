<template>
  <v-select
    v-model="model"
    :items="models"
    item-title="text"
    item-value="value"
    label="Model"
    :rules="[Rule.REQUIRED]"
    required
    hide-details
    @update:model-value="onModelChange"
  />
</template>

<script lang="ts">
export enum RaspberryPiType {
  ZERO = 'Nano',
  ZERO_W = 'ZeroW',
  TWO = 'RP-2',
  THREE = 'RP-3',
  FOUR = 'RP-4',
  FIVE = 'RP-5',
  OTHER = 'Other',
}
</script>
<script lang="ts" setup>
import { ref } from 'vue';
import { Rule } from '@/composables/formComposables';
import { Board } from '@/types/boards';

const board = defineModel<Board>({ required: true });
const model = ref<string>((board.value.model as { RaspberryPi: string }).RaspberryPi);

// List available protocols
const models = Object.keys(RaspberryPiType)
  .filter((key) => isNaN(Number(key))) // Filter out the numeric enum members (reverse mappings)
  .map((key) => ({
    text: (RaspberryPiType as any)[key],
    value: key,
  }));

const onModelChange = (model: string) =>
  ((board.value.model as { RaspberryPi: string }).RaspberryPi = model);
</script>
