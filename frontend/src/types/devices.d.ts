import { BoardId } from '@/types/boards';
import { Branded } from '@/types/hardware';

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
  intensity: number;
  servo_type: Standard | Continuous;
}
