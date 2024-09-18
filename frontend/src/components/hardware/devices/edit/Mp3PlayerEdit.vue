<template>
  <v-card class="mx-auto mb-5" variant="flat">
    <v-card-title>{{ t('files') }}</v-card-title>
    <v-data-table :items="fileList" hide-default-header variant="compact">
      <template #[`item.path`] />
      <template #[`item.actions`]="{ item }">
        <v-btn
          ref="deleteBtn"
          icon="mdi-trash-can"
          size="small"
          variant="text"
          @click="toBeDeleted = item"
        />
      </template>
      <template #no-data>
        <em>{{ t('empty') }}</em>
      </template>
    </v-data-table>
  </v-card>

  <v-alert v-if="error" color="warning" dark variant="tonal" class="mb-2 py-2">
    {{ error }}
  </v-alert>
  <v-file-input
    v-model="currentFile"
    :label="t('file')"
    accept="audio/mpeg"
    prepend-icon="mdi-music"
    :show-size="1000"
    hide-details
    chips
    :class="{ 'mb-7': !progress }"
    @update:model-value="onFileSelect"
  >
    <template #append>
      <v-btn color="success" dark small append-icon="mdi-cloud-upload" @click="onUploadFile">
        {{ t('upload') }}
      </v-btn>
    </template>
  </v-file-input>
  <v-progress-linear v-if="progress" v-model="progress" color="light-blue" height="25" reactive>
    {{ progress }} %
  </v-progress-linear>

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed, onBeforeMount, ref, toValue } from 'vue';
import { useI18n } from 'vue-i18n';
import { useFetchMp3PlayerFileList } from '@/composables/deviceComposables';
import { useConnectionStore } from '@/stores/connectionStore';
import { useToasterStore } from '@/stores/toastStore';
import { Mp3Player } from '@/types/devices';

const { t } = useI18n();
const device = defineModel<Mp3Player>({ required: true });
device.value.pin = 0;
device.value.path = '';

// Compute URLs
const { url: baseUrl, isConnected } = storeToRefs(useConnectionStore());

const fileInfos = ref<{ name: string; path: string }[]>([]);
const currentFile = ref<File | null>(null);
const progress = ref<number>(0);
const error = ref<string>('');

// Compute file list.
const fileList = computed(() => {
  return fileInfos.value.map((file) => {
    return { ...file, actions: 'delete' };
  });
});

// Handle file selection.
const onFileSelect = (file: File | File[]): void => {
  progress.value = 0;
  currentFile.value = Array.isArray(file) ? file[0] : file;
};

const xhr = new XMLHttpRequest();
xhr.upload.addEventListener('progress', function (e) {
  if (e.lengthComputable) {
    progress.value = Math.ceil((e.loaded / e.total) * 100);
  }
});
xhr.onload = async () => {
  if (xhr.status === 200) {
    progress.value = 0;
    error.value = '';
    currentFile.value = null;
    await useFetchMp3PlayerFileList(device.value);
    useToasterStore().success(t('file_uploaded'));
  } else {
    error.value = `Error: ${xhr.status} ${xhr.statusText}`;
  }
};
xhr.onerror = () => {
  error.value = `Error ${xhr.status}: ${xhr.statusText}`;
};

// Handle file selection.
const onUploadFile = () => {
  if (!currentFile.value) {
    error.value = t('no_file_empty');
    return;
  }
  error.value = '';
  const formData = new FormData();
  formData.append('file', toValue(currentFile.value));
  xhr.open(
    'POST',
    toValue(`${baseUrl.value}/api/devices/mp3player/${device.value.id}/file/upload`),
  );
  xhr.send(formData);
};
onBeforeMount(async () => await useFetchMp3PlayerFileList(device.value));

// Delete a file.
const toBeDeleted = ref();
const onConfirmDelete = async () => {
  if (toBeDeleted.value) {
    const data = await fetch(
      `${baseUrl.value}/api/devices/mp3player/${device.value.id}/file/delete`,
      {
        method: 'DELETE',
        body: toBeDeleted.value!.name,
      },
    );
    if (data.ok) {
      await useFetchMp3PlayerFileList(device.value);
      useToasterStore().success(t('file_deleted'));
    }
  }
};
</script>

<i18n>
{
  "en": {
    "file": "Upload new MP3 file",
    "files": "Available files",
    "empty": "No files available",
    "no_file_error": "Please select a file...",
    "upload": "Upload",
    "file_uploaded": "File uploaded",
    "file_deleted": "File Deleted"
  },
  "fr": {
    "file": "Ajouter un fichier Mp3",
    "files": "Fichiers disponibles",
    "empty": "Aucun fichier disponible",
    "no_file_error": "Veuillez sélectionner un fichier...",
    "upload": "Télécharger",
    "file_uploaded": "Fichier téléchargé",
    "file_deleted": "Fichier supprimé"
  }
}
</i18n>
