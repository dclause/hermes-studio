import { Branded } from '@/modules/hardware/types/hardware';
import { DeviceId } from '@/types/devices';
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

  readonly duration: number;
  readonly playing: number;
  readonly progress: number;
}

export declare interface Keyframe {
  device: DeviceId;
  start: number;
  end: number;
  target: number;
  transition: Transition;
}
