<template>
  <!-- Compact variant -->
  <div v-if="variant === 'compact'" class="d-flex flex-1-1-100 align-center mt-2 mb-2">
    <slot name="icon">
      <v-icon class="ml-2 mr-3" icon="mdi-progress-question" size="30" />
    </slot>

    <v-label class="command-label font-weight-bold">
      <slot name="label">
        {{ device.name }}
      </slot>
    </v-label>

    <div class="command-pin ml-2 mr-2 text-lowercase font-italic d-none d-md-block">
      <slot name="info">
        {{ $t('command.pin') }}: {{ device.pin ?? '??' }}
      </slot>
    </div>

    <slot name="command" class="flex-grow-1">
      <div class="font-italic text-error-lighten-1">
        {{ $t('command.none') }}
      </div>
    </slot>
  </div>

  <!-- Normal variant -->
  <!--  <v-card v-else>-->
  <!--    <v-card-title class="d-flex">-->
  <!--      {{ device.name }}-->
  <!--      <v-spacer />-->
  <!--      <slot name="icon">-->
  <!--        <v-icon class="ml-2 mr-3" icon="mdi-progress-question" width="30" />-->
  <!--      </slot>-->
  <!--    </v-card-title>-->
  <!--    <v-card-subtitle>{{ board.name }}</v-card-subtitle>-->
  <!--    <v-card-text>-->
  <!--      <slot name="command">-->
  <!--        <unknown-action />-->
  <!--      </slot>-->
  <!--    </v-card-text>-->
  <!--  </v-card>-->
</template>

<script lang="ts" setup>
import { Actuator } from '@/types/devices';

withDefaults(
  defineProps<{
    variant?: string;
  }>(),
  { variant: 'compact' },
);

const device = defineModel<Actuator>({ required: true });
</script>

<style lang="scss" scoped>
.command-label {
  width: 10rem;
  text-overflow: ellipsis;
  display: block;
}

.command-pin {
  width: 4rem;
}
</style>
