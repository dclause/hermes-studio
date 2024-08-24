<template>
  <v-switch
    v-model="mode"
    class="mx-5"
    inset
    :false-value="HardwareMode.OFF"
    :true-value="HardwareMode.REALTIME"
    :hide-details="true"
  >
    <template #prepend>
      <v-icon
        style="margin-right: -10px"
        icon="mdi-controller-off"
        @click="mode = HardwareMode.OFF"
      />
    </template>
    <template #append>
      <v-icon
        style="margin-left: -10px"
        icon="mdi-controller"
        @click="mode = HardwareMode.REALTIME"
      />
    </template>
  </v-switch>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { watch } from 'vue';
import { useTheme } from 'vuetify';
import { HardwareMode } from '@/composables/globalComposables';
import { useConfigStore } from '@/stores/configurationStore';

const config = useConfigStore();
const { mode } = storeToRefs(config);
const theme = useTheme();

watch(mode, (mode) => {
  switch (mode) {
    case HardwareMode.VIRTUAL:
      theme.global.name.value = 'VirtualModeTheme';
      break;
    case HardwareMode.REALTIME:
      theme.global.name.value = 'RealTimeModeTheme';
      break;
    case HardwareMode.OFF:
      theme.global.name.value = 'OffModeTheme';
      break;
  }
  config.update({ mode });
});
</script>

<style lang="scss">
.mode-switcher {
  opacity: 1 !important;

  .v-slider-track__background--opacity {
    opacity: 1;
  }
}
</style>
