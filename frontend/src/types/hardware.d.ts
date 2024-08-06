// Branded type and flavoring
declare const __brand: unique symbol;

declare interface Brand<B> {
  [__brand]: B;
}

declare type Branded<T, B> = T & Brand<B>;

export declare interface DeviceConfig {
  type: string;

  [x: string]: unknown;
}

export declare type DeviceState = unknown;

export declare type HardwareId = Branded<number, 'HardwareId'>;

export declare interface Device {
  hid: HardwareId;
  name: string;
  config: DeviceConfig;
  state: DeviceState;

  [x: string]: unknown;
}

export declare interface Protocol {
  type: string;

  [x: string]: unknown;
}

export declare type BoardId = Branded<number, 'BoardId'>;

export declare interface Board {
  id: BoardId;
  name: string;
  connected: boolean;
  protocol: Protocol;
  model:
    | string
    | {
        [x: string]: unknown;
      };
}

export declare type StateId = Branded<number, 'StateId'>;

export declare interface State {
  sid: StateId;
  value: DeviceState;
}

export declare type DeviceId = Branded<{ bid: BoardId; hid: HardwareId }, 'DeviceId'>;

export declare type GroupId = Branded<number, 'GroupId'>;

export declare interface Group {
  gid: GroupId;
  name: string;
  devices: DeviceId[];
}
