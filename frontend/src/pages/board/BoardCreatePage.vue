<template>
  <v-card class="mx-auto pa-4" variant="elevated" width="400">
    <v-form
      v-model="isFormValidated"
      :disabled="loading"
      :loading="loading"
      @submit.prevent="onSubmit"
    >
      <!-- Board name -->
      <v-text-field v-model="board.name" label="Name" required :rules="[Rule.REQUIRED]" />
      <!-- Board type -->
      <!--      <v-select-->
      <!--        v-model="board.config.type"-->
      <!--        :items="['Arduino']"-->
      <!--        label="Board type"-->
      <!--        required-->
      <!--        :rules="[Rule.REQUIRED]"-->
      <!--      />-->
      <!-- Board config for arduino -->
      <!--      <v-select-->
      <!--        v-model="board.config.model"-->
      <!--        :items="['NANO', 'UNO', 'MEGA']"-->
      <!--        label="Board model"-->
      <!--        required-->
      <!--        :rules="[Rule.REQUIRED]"-->
      <!--      />-->
      <!-- Port -->
      <!--      <v-text-field-->
      <!--        v-model="board.protocol.port"-->
      <!--        label="Serial port"-->
      <!--        required-->
      <!--        :rules="[Rule.REQUIRED]"-->
      <!--      />-->

      <!-- Submit -->
      <v-btn
        block
        class="mt-2"
        color="primary"
        :disabled="loading"
        :loading="loading"
        size="large"
        type="submit"
        variant="elevated"
      >
        {{ $t('form.create') }}
      </v-btn>
      <v-btn
        block
        class="mt-2"
        color="secondary"
        :disabled="loading"
        :loading="loading"
        size="large"
        variant="elevated"
        @click="router.push('board.list')"
      >
        {{ $t('form.cancel') }}
      </v-btn>
    </v-form>
  </v-card>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { Rule } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useToasterStore } from '@/stores/toastStore';
import { Board } from '@/types/boards';
import { SocketAck } from '@/types/socket';

const { t } = useI18n();
const router = useRouter();

const toaster = useToasterStore();
const boardStore = useBoardStore();
const { loading } = storeToRefs(boardStore);

const isFormValidated = ref<boolean>(false);
const board = boardStore.default();

const onSubmit = () => {
  if (isFormValidated.value) {
    boardStore
      .create(board)
      .then((ack: SocketAck) => {
        if (ack.success) {
          toaster.success(
            t('success', {
              name: (ack.success as Board).name,
              id: (ack.success as Board).id,
            }),
          );
        }
        return router.push({ name: 'board.list' });
      })
      .catch(logError);
  }
};
</script>

<i18n>
{
  "en": {
    "success": "Successfully created board '{name}' [{id}]"
  },
  "fr": {
    "success": "La carte '{name}' [{id}] a été créée avec succès"
  }
}
</i18n>
