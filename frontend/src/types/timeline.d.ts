import Timeline from '@/components/animations/timeline/timeline';
import TimelineRenderer from '@/components/animations/timeline/timeline.renderer.ts_';
import { DeviceId, DeviceState } from '@/types/devices';
import { NestedGroup } from '@/types/groups';

export declare type TimelineItemCallback =
  | ((item: TimelineItem, isAtPosition: boolean, track: Track) => void)
  | null;

export declare interface TimelineEvent {
  topic: string;
  callback: (args: any) => void;
}

export interface Point {
  x: number;
  y: number;
}

export interface Area {
  x1: number;
  y1: number;
  x2: number;
  y2: number;
}

export declare interface HTMLTimelineElement extends HTMLElement {
  timeline?: Timeline;
  renderer?: TimelineRenderer;
}

export declare type TimelineSelection = {
  tracks: Track[];
  keyframes: Keyframe[];
};

export declare type TimelineStyleConfig = {
  colorPrimary: string;
  colorSecondary: string;
  colorHandle: string;
  colorTimeCursor: string;
  colorSelectedHandle: string;
};

export declare type TimelineRenderConfig = {
  scaleFactor: number;
  headerHeight: number;
  trackHeight: number;
  leftMargin: number;
  stepDuration: number;
  stepWidth: number;
  stepSubWidth: number;
  zoomMin: number;
  zoomMax: number;
  zoomSpeed: number;
};

export declare type TimelineConfig = TimelineRenderConfig & TimelineStyleConfig;
export declare type PartialTimelineConfig = Partial<TimelineRenderConfig> &
  Partial<TimelineStyleConfig>;

export declare interface TimelineItem {
  hovered: boolean;
  selected: boolean;
  move_start_position: number;
  move_previous_position: number;
  resize_start_position: number;
  resize_previous_position: number;

  [x: string]: any;
}

export declare type TrackId = Branded<number, 'TrackId'>;

export declare interface Track extends NestedGroup {
  open?: boolean;
  disabled?: boolean;
}

export declare interface Keyframe {
  id: KeyFrameId;
  track: TrackId;
  time: number;
  duration: number;
  data: KeyFrameData[];
}

export declare interface KeyFrameData {
  device: DeviceId;
  state: DeviceState;
}
