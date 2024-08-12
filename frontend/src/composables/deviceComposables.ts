import { Component } from 'vue';
import Led from '@/components/hardware/actuators/Led.vue';
import Servo from '@/components/hardware/actuators/Servo.vue';

export enum DeviceType {
  Led = 'Led',
  Servo = 'Servo',
}

export const useDeviceTypes = (): Record<DeviceType, Component> => {
  return {
    [DeviceType.Led]: Led,
    [DeviceType.Servo]: Servo,
  };
};

export const useDeviceComponent = (type: DeviceType): Component | undefined => {
  return useDeviceTypes()[type];
};
