<template>
  <v-card class="wrapper d-flex flex-1-1-100 align-center my-2">
    <slot name="prefix" />
    <!-- Compact variant -->
    <div v-if="variant === 'compact'" class="d-flex flex-1-1-100 align-center mt-2 mb-2 command">
      <div class="d-none d-sm-block">
        <slot name="icon">
          <v-icon class="mx-2" icon="mdi-progress-question" size="30" />
        </slot>
      </div>

      <v-label class="command-label font-weight-bold ml-2">
        <slot name="label">
          {{ device.name }}
        </slot>
      </v-label>

      <div class="command-pin ml-2 mr-2 text-lowercase font-italic d-none d-sm-block">
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

    <v-btn
      icon="mdi-pencil"
      size="small"
      :to="{
        name: 'device.edit',
        params: { id: device.id },
      }"
      variant="text"
    />

    <v-btn icon="mdi-trash-can" size="small" variant="text" @click="emit('delete', device)" />
  </v-card>
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

const emit = defineEmits<{ delete: [item: Actuator] }>();
const device = defineModel<Actuator>({ required: true });
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

.command-pin {
  width: 4rem;
}
</style>
