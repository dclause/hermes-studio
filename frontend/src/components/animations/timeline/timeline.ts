import { HTMLTimelineElement, Point, TimelineEvent, Track } from '@/types/timeline';

export default class Timeline {
  /** Component container */
  protected _container: HTMLTimelineElement | null = null;
  /** Stores the subscribed event handlers external component have registered to the timeline. */
  protected _eventSubscriptions: TimelineEvent[] = [];
  /** Stores the last time a click was done (usefull to detect doubleclick events) */
  protected _lastClickTime: number = 0;
  /** Stores the mouse position on canvas (usefull to know if the user is over the canvas */
  protected _mousePositionOnCanvas: Point = { x: 0, y: 0 };
  /** History for tracks */
  protected _tracksHistoryIndex: number = -1;
  protected _tracksHistory: Array<Track[]> = [];
}
