import { useBoardStore } from '@/stores/boardStore';
import { useExpanderStore } from '@/stores/expanderStore';
import { Board, BoardId, ExpanderId, Hardware, HardwareId } from '@/types/hardwares';

export enum HardwareType {
  Unknown = '',
  Device = 'Device',
  Board = 'Board',
  Expander = 'Expander',
}

export function useHardware() {
  const boardStore = useBoardStore();
  const expanderStore = useExpanderStore();

  /**
   * Get the hardware associated with the give HardwareId.
   */
  const get_hardware = (hid: HardwareId): Hardware | undefined => {
    switch (hid.type) {
      case HardwareType.Board:
        return boardStore.get(hid.id as BoardId);
      case HardwareType.Expander:
        return expanderStore.get(hid.id as ExpanderId);
      default:
        return undefined;
    }
  };

  /**
   * Given a HardwareId; recursively follow the expander hierarchy until the parent board is found.
   */
  const get_board = (hid: HardwareId): Board => {
    let hardware_id: HardwareId | undefined = hid;
    let hardware = null;
    while (hardware_id) {
      hardware = get_hardware(hardware_id);
      hardware_id = hardware && 'hid' in hardware ? hardware.hid : undefined;
    }

    return hardware as Board;
  };

  return { get_hardware, get_board };
}
