import { Branded } from '@/modules/hardware/types/hardware';
import { DeviceId, DeviceState } from '@/types/devices';
import { GroupId } from '@/types/groups';

export declare type AnimationId = Branded<number, 'AnimationId'>;

export declare interface Animation {
  id: AnimationId;
  name: string;
  description: string;
  repeat: bool;
  loopback: number;
  fps: number;
  speed: number;
  tracks: Record<GroupId, Keyframe[]>;

  duration: number;
  playing: number;
  progress: number;
}

export declare interface Keyframe {
  start: number;
  end: number;
  transition: Transition;
  positions: Position[];
}

export declare interface Position {
  device: DeviceId;
  target: DeviceState;
}
