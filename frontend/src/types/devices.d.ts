import type { BoardId } from '@/types/boards';
import type { Branded, Entity, Range } from '@/types/core';
import { DeviceType } from '@/composables/deviceComposables';

export declare type DeviceId = Branded<number, 'DeviceId'>;
export declare type DeviceState = number;

export declare interface Device extends Entity<DeviceId> {
  type: DeviceType;
  bid: BoardId;

  [x: string]: unknown;
}

export declare interface Actuator extends Device {
  default: DeviceState;
  state: DeviceState;
}

export declare interface Led extends Actuator {
  pin: number;
  intensity: number;
}

export declare interface Servo extends Actuator {
  pin: number;
  servo_type: Standard | Continuous;
  range: Range<number>;
  pwm_range: Range<number>;
  degree_range: Range<number>;
}
