import { Branded } from '@/modules/hardware/types/hardware';

export declare type AnimationId = Branded<number, 'AnimationId'>;

export declare interface Animation {
  id: AnimationId;
  name: string;
  description: string;
}
