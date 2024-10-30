import type { BoardId } from '@/types/boards';
import type { Branded, Entity, Range } from '@/types/core';
import { DeviceType } from '@/composables/deviceComposables';

export declare type DeviceId = Branded<number, 'DeviceId'>;
export declare type DeviceState = unknown;

export declare type Device = Entity<DeviceId> & {
  type: keyof typeof DeviceType;
  bid: BoardId;

  [x: string]: unknown;
};

export declare type OutputDevice = Device & {
  default: DeviceState;
  state: DeviceState;
};

export declare type Led = OutputDevice & {
  pin: number;
  brightness: number;
};

export declare type Servo = OutputDevice & {
  pin: number;
  servo_type: 'Standard' | 'Continuous';
  range: Range<number>;
  pwm_range: Range<number>;
  degree_range: Range<number>;
  inverted: boolean;
  auto_detach: boolean;
  detach_delay: number;
};

export declare type Mp3Player = OutputDevice & {
  path: string;
};

export declare type Mp3PlayerFile = {
  name: string;
  path: string;
};

export declare type Mp3PlayerState = {
  path: string;
  status: number;
};
