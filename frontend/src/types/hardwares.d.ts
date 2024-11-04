import { ProtocolType } from '@/composables/boardComposables';
import { Entity } from '@/types/core';
import { Branded } from '@/types/hardware';

export declare interface Protocol {
  type: keyof typeof ProtocolType;
  transport?: Transport;

  [x: string]: unknown;
}

export declare interface Transport {
  type: string;

  [x: string]: unknown;
}

export declare type HardwareId = Branded<number, 'BoardId'>;
export declare type BoardModel =
  | string
  | {
      [x: string]: unknown;
    };

export declare interface Board extends Entity<HardwareId> {
  connected: boolean;
  protocol: Protocol;
  model: BoardModel;
}
