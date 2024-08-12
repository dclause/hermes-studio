import type { BoardId } from '@/types/boards';
import type { Branded, Range } from '@/types/core';

export declare type DeviceId = Branded<number, 'DeviceId'>;
export declare type DeviceState = number;

export declare interface Actuator {
  id: DeviceId;
  name: string;
  type: DeviceType;
  bid: BoardId;
  default: DeviceState;
  state: DeviceState;

  [x: string]: unknown;
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
