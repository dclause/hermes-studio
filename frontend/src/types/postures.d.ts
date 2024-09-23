import { Entity } from '@/types/core';

export declare type PostureId = Branded<number, 'PostureId'>;

export declare type Posture = Entity<PostureId> & {
  description: string;
  positions: Position[];
};
