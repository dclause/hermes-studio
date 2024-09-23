import { storeToRefs } from 'pinia';
import { Component, computed } from 'vue';
import { useTheme } from 'vuetify';
import ArduinoBoardEdit from '@/components/hardware/boards/edit/ArduinoBoardEdit.vue';
import DefaultBoardEdit from '@/components/hardware/boards/edit/DefaultBoardEdit.vue';
import RaspberryPiBoardEdit from '@/components/hardware/boards/edit/RaspberryPiBoardEdit.vue';
import DefaultProtocolEdit from '@/components/hardware/protocols/edit/DefaultProtocolEdit.vue';
import RaspiProtocolEdit from '@/components/hardware/protocols/edit/RaspiProtocolEdit.vue';
import SerialProtocolEdit from '@/components/hardware/protocols/edit/SerialProtocolEdit.vue';
import { useBoardStore } from '@/stores/boardStore';
import { BoardModel } from '@/types/boards';

export enum RobotStatus {
  OFF = 0,
  PARTIAL = 1,
  ON = 2,
}

export enum BoardType {
  Unknown = '',
  Arduino = 'Arduino',
  RaspberryPi = 'RaspberryPi',
}

export const useBoardModelType = (type: BoardModel): BoardType => {
  switch (typeof type) {
    case 'string':
      return type as BoardType;
    case 'object':
      return Object.keys(type)[0] as BoardType;
    default:
      return BoardType.Unknown;
  }
};

export const useBoardModelEditComponent = (model: BoardModel): Component | undefined => {
  const mapping = {
    [BoardType.Unknown]: DefaultBoardEdit,
    [BoardType.Arduino]: ArduinoBoardEdit,
    [BoardType.RaspberryPi]: RaspberryPiBoardEdit,
  };
  return mapping[useBoardModelType(model)];
};

// -------------------------------------

export enum ProtocolType {
  UnknownProtocol = 'Unknown',
  SerialProtocol = 'Serial protocol',
  RaspiProtocol = 'Raspi protocol',
}

export const useProtocolEditComponent = (
  protocol: keyof typeof ProtocolType,
): Component | undefined => {
  const mapping = {
    [ProtocolType.UnknownProtocol]: DefaultProtocolEdit,
    [ProtocolType.SerialProtocol]: SerialProtocolEdit,
    [ProtocolType.RaspiProtocol]: RaspiProtocolEdit,
  };
  return mapping[ProtocolType[protocol]];
};

export function useBoardMode() {
  const theme = useTheme();
  const { boards } = storeToRefs(useBoardStore());
  return computed(() => {
    const all_boards = Object.values(boards.value);
    const connected_boards = all_boards.filter((board) => board.connected);
    if (connected_boards.length === 0) {
      theme.global.name.value = 'OffModeTheme';
      return RobotStatus.OFF;
    } else if (connected_boards.length === all_boards.length) {
      theme.global.name.value = 'OnModeTheme';
      return RobotStatus.ON;
    } else {
      theme.global.name.value = 'PartialModeTheme';
      return RobotStatus.PARTIAL;
    }
  });
}
