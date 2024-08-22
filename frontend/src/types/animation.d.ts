import { Branded } from '@/modules/hardware/types/hardware';
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
  keyframes: Record<GroupId, Keyframe[]>;
}

export declare interface Keyframe {
  start: number;
  end: number;
  target: number;
  transition: Transition;
}
