import { Component } from 'vue';
import DefaultActuator from '@/components/hardware/actuators/DefaultActuator.vue';
import DefaultDeviceEdit from '@/components/hardware/actuators/edit/DefaultDeviceEdit.vue';
import LedEdit from '@/components/hardware/actuators/edit/LedEdit.vue';
import ServoEdit from '@/components/hardware/actuators/edit/StandardServoEdit.vue';
import Led from '@/components/hardware/actuators/Led.vue';
import Servo from '@/components/hardware/actuators/Servo.vue';

export enum DeviceType {
  Unknown = '',
  Led = 'Led',
  Servo = 'Servo',
}

export const useDeviceComponent = (type: DeviceType): Component | undefined => {
  const mapping = {
    [DeviceType.Unknown]: DefaultActuator,
    [DeviceType.Led]: Led,
    [DeviceType.Servo]: Servo,
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
