import { Component } from 'vue';
import DefaultActuator from '@/components/hardware/actuators/DefaultActuator.vue';
import Led from '@/components/hardware/actuators/Led.vue';
import Servo from '@/components/hardware/actuators/Servo.vue';

export enum DeviceType {
  Unknown = 'Unknown',
  Led = 'Led',
  Servo = 'Servo',
}

export const useDeviceTypes = (): Record<DeviceType, Component> => {
  return {
    [DeviceType.Unknown]: DefaultActuator,
    [DeviceType.Led]: Led,
    [DeviceType.Servo]: Servo,
  };
};

export const useDeviceComponent = (type: DeviceType): Component | undefined => {
  return useDeviceTypes()[type];
};
