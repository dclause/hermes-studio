import { TimelineEvents } from '@/components/animations/timeline/timeline.events';
import TimelineRenderer from '@/components/animations/timeline/timeline.renderer';
import TimelineUtils from '@/components/animations/timeline/timeline.utils';
import { Keyframe } from '@/types/animation';
import { DeviceId } from '@/types/devices';
import {
  HTMLTimelineElement,
  Point,
  TimelineConfig,
  TimelineEvent,
  TimelineItem,
  Track,
} from '@/types/timeline';

export default class Timeline extends TimelineRenderer {
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

  /**
   * Initializes Timeline.
   */
  init(container: HTMLElement, tracks: Track[], config: TimelineConfig) {
    this._container = container;

    super.init(container, tracks, config);
    this._container.timeline = this;
    this._bindEvents();

    this.setTracks(tracks);
    this._pushToHistory();
  }

  destroy = (): void => {
    this._unbindEvents();
    this._container = null;
    super.destroy();
  };

  /**
   * Rescale, update size of the container and render it.
   */
  refresh = () => {
    this.rescale();
    if (window?.requestAnimationFrame) {
      window.requestAnimationFrame(() => this.render());
    } else {
      this.render();
    }
  };

  setTime = (timeInMs: number) => {
    this._currentTime = timeInMs;
    this._emit(TimelineEvents.updateTime, timeInMs);
    this.render();
  };

  // ####################
  // Event handling.
  // ####################

  protected _bindEvents = (): void => {
    this._container?.addEventListener('resize', this._onResizeEvent);
    this._container?.addEventListener('wheel', this._onWheelEvent, { passive: false });
    this._container?.addEventListener('touchstart', this._onMouseDown);
    this._container?.addEventListener('mousedown', this._onMouseDown);
    window?.addEventListener('keydown', this._onKeyDown);
    window?.addEventListener('resize', this._onResizeEvent);
    window?.addEventListener('touchmove', this._onMouseMove);
    window?.addEventListener('mousemove', this._onMouseMove);
    window?.addEventListener('touchend', this._onMouseUp);
    window?.addEventListener('mouseup', this._onMouseUp);
    this._scrollContainer?.addEventListener('scroll', this._onScrollEvent);
  };

  protected _unbindEvents = (): void => {
    this._container?.removeEventListener('resize', this._onResizeEvent);
    this._container?.removeEventListener('wheel', this._onWheelEvent);
    this._container?.removeEventListener('touchstart', this._onMouseDown);
    this._container?.removeEventListener('mousedown', this._onMouseDown);
    window?.removeEventListener('keydown', this._onKeyDown);
    window?.removeEventListener('resize', this._onResizeEvent);
    window?.removeEventListener('touchmove', this._onMouseMove);
    window?.removeEventListener('mousemove', this._onMouseMove);
    window?.removeEventListener('touchend', this._onMouseUp);
    window?.removeEventListener('mouseup', this._onMouseUp);
    this._scrollContainer?.removeEventListener('scroll', this._onScrollEvent);
  };

  /**
   * Subscribe to a timeline event: this method helps externals (such as Vue components) to react
   * to changes (see TimelineEvents) within the timeline.
   * @param topic event name.
   * @param callback callback to be added.
   */
  on(topic: string, callback: (...args: any[]) => void): boolean {
    // Check if the callback is already registered for the given topic
    const exists = this._eventSubscriptions.some(
      (event) => event.topic === topic && event.callback === callback,
    );

    // If it doesn't exist, add the new subscription
    if (!exists) {
      this._eventSubscriptions.push({ topic, callback });
      return true;
    }

    // Callback already exists
    return false;
  }

  /**
   * Emit event: Let the event emit one of the possible events (see TimelineEvents) and call all
   * externally registered associated callbacks (see `Timeline.on()`).
   * @param topic Event name.
   * @param args Event arguments.
   */
  protected _emit(topic: string, ...args: any[]): void {
    for (const event of this._eventSubscriptions) {
      if (event?.topic === topic && event?.callback) {
        event.callback(...(args as [any]));
      }
    }
  }

  /**
   * Handler for container resize event: rebuild the timeline to fit the new container size.
   */
  protected _onResizeEvent = (event: Event): void => {
    (event.currentTarget as HTMLTimelineElement)?.timeline?.refresh();
  };

  /**
   * Handler for wheel event: timeline will either scroll or zoom depending on the ctrl-key status.
   */
  protected _onWheelEvent = (event: WheelEvent): void => {
    if (event.ctrlKey) {
      event.preventDefault();
      (event.currentTarget as HTMLTimelineElement).timeline?.onZoom(event);
    }
  };

  /**
   * Handler for the scroll (from scrollbar) event.
   */
  protected _onScrollEvent = (event: Event): void => {
    event.preventDefault();
    const timeline = (event.target as HTMLElement)?.parentElement as HTMLTimelineElement;
    timeline?.timeline?.refresh();
    this._emit(TimelineEvents.scroll, timeline?.timeline?._scrollContainer?.scrollTop);
  };

  protected _onMouseDown = (event: TouchEvent | MouseEvent) => {
    event.preventDefault();
    let hasSelection = false;
    const isDoubleClick = Date.now() - this._lastClickTime < 400;
    this._mousePositionOnCanvas = this._getMousePositionOnCanvas(event);

    this._selectionArea = null;
    this.applyAtPosition(
      this._mousePositionOnCanvas,
      (item: TimelineItem, isAtPosition: boolean) => {
        if (!event.ctrlKey) {
          // Toggle the current keyframe (keep selected the others only if ctrlKey)
          item.selected = isAtPosition && !item.selected;
        }
        hasSelection = hasSelection || item.selected;
      },
      // Set the current position of the keyframe when clicked on: this allows moving it later on.
      (item: TimelineItem, isAtPosition: boolean) => {
        if (isAtPosition) {
          this._selectionArea = null;
        }
        if (item.selected || isAtPosition) {
          item.move_start_position = item.move_previous_position = this._mousePositionOnCanvas.x;
        }
      },
      (item: TimelineItem, isAtPosition: boolean) => {
        if (isAtPosition) {
          item.resize_start_position = item.resize_previous_position =
            this._mousePositionOnCanvas.x;
        }
      },
      // Creates a keyframe if double-click on the track.
      (item: TimelineItem, isAtPosition: boolean) => {
        if (isAtPosition) {
          if (isDoubleClick) {
            this._onCreateKeyframe(
              item as unknown as Track,
              this.pxPositionToVal(this._mousePositionOnCanvas.x),
            );
          } else {
            this._selectionArea = {
              x1: this._mousePositionOnCanvas.x,
              y1: this._mousePositionOnCanvas.y,
              x2: this._mousePositionOnCanvas.x,
              y2: this._mousePositionOnCanvas.y,
            };
          }
        }
      },
    );
    if (!hasSelection) {
      // this._emit(TimelineEvents.selectKeyframe, null);
    }
    this.render();
    this._lastClickTime = Date.now();
  };

  protected _onMouseMove = (event: TouchEvent | MouseEvent) => {
    event.preventDefault();
    let offsetX = 0;
    const scrollLeft = this._scrollContainer!.scrollLeft;
    this._mousePositionOnCanvas = this._getMousePositionOnCanvas(event);

    // When shiftkey is used: draw a selection box.
    if (this._selectionArea) {
      this._selectionArea.x2 = this._mousePositionOnCanvas.x;
      this._selectionArea.y2 = this._mousePositionOnCanvas.y;
      this.applyAtPosition(this._selectionArea, (item: TimelineItem, isAtPosition: boolean) => {
        item.hovered = isAtPosition;
      });
    } else {
      this._selectionArea = null;
      let maxTime = 0;
      this.applyAtPosition(
        this._mousePositionOnCanvas,
        // Set element to hovered when it is clicked on.
        (item: TimelineItem, isAtPosition: boolean) => {
          item.hovered = isAtPosition;
        },
        (item: TimelineItem, isAtPosition: boolean, track: Track) => {
          const keyframe = item as unknown as Keyframe;
          const duration = keyframe.end - keyframe.start;
          if (item.move_previous_position) {
            offsetX = this._mousePositionOnCanvas.x - item.move_previous_position;
            const newTime = Math.max(0, keyframe.start + this.pxToVal(offsetX));
            const intersectingKeyframe = track.keyframes
              .filter((kf) => kf.start !== keyframe.start)
              .find((kf) =>
                TimelineUtils.isIntersect(
                  { x1: kf.start, y1: 0, x2: kf.end, y2: 50 },
                  {
                    x1: newTime,
                    y1: 0,
                    x2: newTime + duration,
                    y2: 50,
                  },
                ),
              );
            if (!intersectingKeyframe) {
              keyframe.start = newTime;
              keyframe.end = keyframe.start + duration;
              item.move_previous_position = this._mousePositionOnCanvas.x;
            }
          }
          maxTime = Math.max(maxTime, keyframe.end);
        },
        (item: TimelineItem, isAtPosition: boolean, track: Track) => {
          const keyframe = item as unknown as Keyframe;
          const duration = keyframe.end - keyframe.start;
          if (item.resize_previous_position) {
            offsetX = this._mousePositionOnCanvas.x - item.resize_previous_position;
            const newDuration = Math.max(100, duration + this.pxToVal(offsetX));
            const intersectingKeyframe = track.keyframes
              .filter((kf) => kf.start !== keyframe.start)
              .find((kf) =>
                TimelineUtils.isIntersect(
                  { x1: kf.start, y1: 0, x2: kf.end, y2: 50 },
                  {
                    x1: keyframe.start,
                    y1: 0,
                    x2: keyframe.start + newDuration,
                    y2: 50,
                  },
                ),
              );
            if (!intersectingKeyframe) {
              keyframe.end = keyframe.start + newDuration;
              item.resize_previous_position = this._mousePositionOnCanvas.x;
            }
          }
        },
      );

      this._totalDuration = maxTime;
    }
    this.rescale();
    // Scroll the container if the keyframe becomes outside the viewport.
    const newLeft = scrollLeft + offsetX;
    if (
      offsetX > 0 &&
      newLeft + this._canvas!.clientWidth >= this._scrollContainer!.scrollWidth - 5
    ) {
      this._scrollContainer!.scrollLeft = this._scrollContainer!.scrollWidth;
    } else if (offsetX < 0) {
      this._scrollContainer!.scrollLeft = newLeft;
    }
    this.render();
  };

  protected _onMouseUp = (event: TouchEvent | MouseEvent) => {
    event.preventDefault();
    let needHistory = false;

    if (this._selectionArea) {
      this.applyAtPosition(this._selectionArea, (item: TimelineItem, isAtPosition: boolean) => {
        item.selected = isAtPosition;
      });
      this._selectionArea = null;
    } else {
      this._mousePositionOnCanvas = this._getMousePositionOnCanvas(event);
      this.applyAtPosition(
        this._mousePositionOnCanvas,
        (item: TimelineItem, isAtPosition: boolean, track: Track) => {
          if (event.ctrlKey) {
            // Toggle the current keyframe (keep selected the others only if ctrlKey)
            item.selected = (isAtPosition && !item.selected) || (!isAtPosition && item.selected);
          } else if (item.selected) {
            this._emit(TimelineEvents.selectKeyframe, item as unknown as Keyframe, track);
          }
        },
        (item: TimelineItem) => {
          if (item.move_start_position != item.move_previous_position) {
            needHistory = true;
          }
          item.move_start_position = item.move_previous_position = 0;
        },
        (item: TimelineItem) => {
          if (item.resize_start_position != item.resize_previous_position) {
            needHistory = true;
          }
          item.resize_start_position = item.resize_previous_position = 0;
        },
      );
    }
    this.render();

    if (needHistory) {
      this._pushToHistory();
    }
  };

  /** Handle keyboard events (Ctrl-C, Ctrl-V, Ctrl-Z, Delete, etc..) */
  protected _onKeyDown = async (event: KeyboardEvent) => {
    // Only act when the mouse is within the canvas.
    if (
      this._ctx &&
      this._mousePositionOnCanvas.x >= 0 &&
      this._mousePositionOnCanvas.x <= this._ctx!.canvas.clientWidth &&
      this._mousePositionOnCanvas.y >= 0 &&
      this._mousePositionOnCanvas.y <= this._ctx!.canvas.height
    ) {
      // Delete selected items
      if (event.key === 'Backspace' || event.key === 'Delete') {
        event.preventDefault();
        const deleteKeyframeToTracks = (tracks: Track[]) => {
          for (const track of tracks) {
            track.keyframes = track.keyframes.filter(
              (item) => !(item as unknown as TimelineItem).selected,
            );
            deleteKeyframeToTracks(track.children);
          }
        };
        deleteKeyframeToTracks(this._tracks);
        this._emit(TimelineEvents.selectKeyframe, null);
        this._pushToHistory();
        return;
      }

      // Ctrl+C: copy all selected items in the buffer.
      if (event.ctrlKey && event.key === 'c') {
        event.preventDefault();
        const keyframesToCopy: TimelineItem[] = [];
        this.applyAtPosition(
          this._mousePositionOnCanvas,
          null,
          (item: TimelineItem, _, track: Track) => {
            if (item.selected) {
              keyframesToCopy.push({
                ...item,
                start: item.end,
                end: (item.end as number) + (item.end as number) - (item.start as number),
                type: 'keyframe',
                track: track.id,
              });
            }
          },
        );
        await navigator.clipboard.writeText(JSON.stringify(keyframesToCopy));
        return;
      }

      // Ctrl+V: paste all selected items from the buffer.
      if (event.ctrlKey && event.key === 'v') {
        event.preventDefault();
        // Read and check data from the clipboard.
        const keyframesToPaste = JSON.parse(await navigator.clipboard.readText());
        if (
          !Array.isArray(keyframesToPaste) ||
          !keyframesToPaste.every((item) => item.type === 'keyframe')
        )
          return false;

        const addKeyframeToTracks = (tracks: Track[]) => {
          for (const track of tracks) {
            track.keyframes.map((item) => ((item as unknown as TimelineItem).selected = false));
            track.keyframes.push(
              ...keyframesToPaste.filter((keyframe) => keyframe.track === track.id),
            );
            addKeyframeToTracks(track.children);
          }
        };
        addKeyframeToTracks(this._tracks);
        this._pushToHistory();
        return;
      }

      // Ctrl+Z: cancel last action from history.
      if (event.ctrlKey && event.key === 'z') {
        this._backInHistory();
      }

      // Ctrl+Y: repeat last action from history.
      if (event.ctrlKey && event.key === 'y') {
        this._forwardInHistory();
      }
    }
  };

  /** Create a keyframe on the given track, at the appropriate time. */
  protected _onCreateKeyframe = (track: Track, time: number) => {
    const lastKeyframe = track.keyframes.reduce((lastKeyframe: Keyframe | null, keyframe) => {
      if (keyframe.start < time && keyframe.start > (lastKeyframe?.start ?? -1)) {
        return keyframe;
      }
      return lastKeyframe;
    }, null);

    track.keyframes.push({
      start: time,
      end: time + 200,
      transition: lastKeyframe?.transition ?? 'Linear',
      positions: lastKeyframe?.positions ?? [
        {
          device: track.device ?? (track.id as DeviceId),
          target: 0,
        },
      ],
    });
    this._pushToHistory();
  };

  // #################################
  // HISTORY
  // #################################

  /** Saves the current tracks into the history (see Ctrl-Z action) */
  protected _pushToHistory() {
    const length = this._tracks.length;
    this._tracksHistoryIndex++;
    // Slice the history to keep only the last 50 actions.
    this._tracksHistory = this._tracksHistory.slice(
      Math.max(length - 50, 0),
      this._tracksHistoryIndex,
    );
    const backupTracks = TimelineUtils.deepClone(this._tracks);
    this._tracksHistory.push(backupTracks);
    this._emit(TimelineEvents.updateTracks, backupTracks);
  }

  protected _backInHistory() {
    if (this._tracksHistoryIndex > 0) {
      this._tracksHistoryIndex--;
      const lastTracks = this._tracksHistory.at(this._tracksHistoryIndex);
      if (lastTracks) {
        this.setTracks(TimelineUtils.deepClone(lastTracks));
      }
    }
  }

  protected _forwardInHistory() {
    if (this._tracksHistoryIndex < this._tracksHistory.length - 1) {
      this._tracksHistoryIndex++;
      const nextTracks = this._tracksHistory.at(this._tracksHistoryIndex);
      if (nextTracks) {
        this.setTracks(TimelineUtils.deepClone(nextTracks));
      }
    }
  }
}
