<template>
  <v-select
    v-model="model"
    :items="mapEnumToOptions(ArduinoType)"
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
export enum ArduinoType {
  NANO = 'NANO',
  UNO = 'UNO',
  MEGA = 'MEGA',
  OTHER = 'OTHER',
}
</script>
<script lang="ts" setup>
import { ref } from 'vue';
import { Rule } from '@/composables/formComposables';
import { mapEnumToOptions } from '@/composables/globalComposables';
import { Board } from '@/types/boards';

const board = defineModel<Board>({ required: true });
const model = ref<string>((board.value.model as { Arduino: string }).Arduino);

const onModelChange = (model: string) =>
  ((board.value.model as { Arduino: string }).Arduino = model);
</script>
