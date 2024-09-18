<template>
  <v-select
    v-model="model"
    :items="mapEnumToOptions(RaspberryPiType)"
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
import { mapEnumToOptions } from '@/composables/globalComposables';
import { Board } from '@/types/boards';

const board = defineModel<Board>({ required: true });
const model = ref<string>((board.value.model as { RaspberryPi: string }).RaspberryPi);

const onModelChange = (model: string) =>
  ((board.value.model as { RaspberryPi: string }).RaspberryPi = model);
</script>
