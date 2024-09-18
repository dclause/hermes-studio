import { ProtocolType } from '@/composables/boardComposables';
import { Entity } from '@/types/core';
import { Branded } from '@/types/hardware';

export declare interface Protocol {
  type: keyof typeof ProtocolType;

  [x: string]: unknown;
}

export declare type BoardId = Branded<number, 'BoardId'>;
export declare type BoardModel =
  | string
  | {
      [x: string]: unknown;
    };

export declare interface Board extends Entity<BoardId> {
  connected: boolean;
  protocol: Protocol;
  model: BoardModel;
}
