import { ProtocolType } from '@/composables/boardComposables';
import { ExpanderType } from '@/composables/expanderComposables';
import { HardwareType } from '@/composables/hardwareComposables';
import { Branded, Entity } from '@/types/core';

export declare interface Protocol {
  type: keyof typeof ProtocolType;
  transport?: Transport;

  [x: string]: unknown;
}

export declare interface Transport {
  type: string;

  [x: string]: unknown;
}

export declare type HardwareId = Branded<
  {
    type: HardwareType;
    id: number;
  },
  'HardwareId'
>;

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

export declare type ExpanderId = Branded<number, 'ExpanderId'>;

export declare interface Expander extends Entity<ExpanderId> {
  type: keyof typeof ExpanderType;
  hid: HardwareId;

  [x: string]: unknown;
}

export declare type PCA9685 = Expander & {
  address: number;
};

export declare type Hardware = Board | Expander | Device;
