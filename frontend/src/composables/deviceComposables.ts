import { storeToRefs } from 'pinia';
import { Component } from 'vue';
import DefaultCommand from '@/components/hardware/device/commands/DefaultCommand.vue';
import LedCommand from '@/components/hardware/device/commands/LedCommand.vue';
import Mp3PlayerCommand from '@/components/hardware/device/commands/Mp3PlayerCommand.vue';
import ServoCommand from '@/components/hardware/device/commands/ServoCommand.vue';
import DefaultDeviceEdit from '@/components/hardware/device/edit/DefaultDeviceEdit.vue';
import LedEdit from '@/components/hardware/device/edit/LedEdit.vue';
import Mp3PlayerEdit from '@/components/hardware/device/edit/Mp3PlayerEdit.vue';
import ServoEdit from '@/components/hardware/device/edit/StandardServoEdit.vue';
import { useConnectionStore } from '@/stores/connectionStore';
import { Device, Mp3PlayerFile } from '@/types/devices';

export enum DeviceType {
  Unknown = '',
  Led = 'Led',
  Servo = 'Servo',
  Mp3Player = 'Mp3 Player',
}

export const useDeviceComponent = (type: keyof typeof DeviceType): Component | undefined => {
  const mapping = {
    [DeviceType.Unknown]: DefaultCommand,
    [DeviceType.Led]: LedCommand,
    [DeviceType.Servo]: ServoCommand,
    [DeviceType.Mp3Player]: Mp3PlayerCommand,
  };
  return mapping[DeviceType[type]];
};

export const useDeviceEditComponent = (type: keyof typeof DeviceType): Component | undefined => {
  const mapping = {
    [DeviceType.Unknown]: DefaultDeviceEdit,
    [DeviceType.Led]: LedEdit,
    [DeviceType.Servo]: ServoEdit,
    [DeviceType.Mp3Player]: Mp3PlayerEdit,
  };
  return mapping[DeviceType[type]];
};

// Fetch the available file list.
export const useFetchMp3PlayerFileList = async (device: Device): Promise<Mp3PlayerFile[]> => {
  const { url, isConnected } = storeToRefs(useConnectionStore());
  if (isConnected.value) {
    const data = await fetch(`${url.value}/api/devices/mp3player/${device.id}/files`);
    if (data.ok) {
      return [{ path: '', name: '---' }, ...((await data.json()) as Mp3PlayerFile[])];
    }
  }
  return [];
};
