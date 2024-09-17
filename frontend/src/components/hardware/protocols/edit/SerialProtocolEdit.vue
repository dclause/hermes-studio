<template>
  <v-text-field
    v-model="protocol.port"
    label="Port"
    :rules="[Rule.REQUIRED]"
    required
    hide-details
  />
</template>

<script lang="ts" setup>
import { Rule } from '@/composables/formComposables';
import { Protocol } from '@/types/boards';

const protocol = defineModel<Protocol>({ required: true });
</script>
