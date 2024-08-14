import { Component } from 'vue';
import DefaultCommand from '@/components/hardware/devices/commands/DefaultCommand.vue';
import LedCommand from '@/components/hardware/devices/commands/LedCommand.vue';
import ServoCommand from '@/components/hardware/devices/commands/ServoCommand.vue';
import DefaultDeviceEdit from '@/components/hardware/devices/edit/DefaultDeviceEdit.vue';
import LedEdit from '@/components/hardware/devices/edit/LedEdit.vue';
import ServoEdit from '@/components/hardware/devices/edit/StandardServoEdit.vue';

export enum DeviceType {
  Unknown = '',
  Led = 'Led',
  Servo = 'Servo',
}

export const useDeviceComponent = (type: DeviceType): Component | undefined => {
  const mapping = {
    [DeviceType.Unknown]: DefaultCommand,
    [DeviceType.Led]: LedCommand,
    [DeviceType.Servo]: ServoCommand,
  };
  return mapping[type];
};

export const useDeviceEditComponent = (type: DeviceType): Component | undefined => {
  const mapping = {
    [DeviceType.Unknown]: DefaultDeviceEdit,
    [DeviceType.Led]: LedEdit,
    [DeviceType.Servo]: ServoEdit,
  };
  return mapping[type];
};
