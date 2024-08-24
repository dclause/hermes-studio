import TimelineDrawing from '@/components/animations/timeline/timeline.drawing';
import TimelineUtils from '@/components/animations/timeline/timeline.utils';
import {
  Area,
  Point,
  TimelineConfig,
  TimelineItem,
  TimelineItemCallback,
  Track,
} from '@/types/timeline';

export default abstract class TimelineRenderer extends TimelineDrawing {
  /** Tracks (contains the keyframes) */
  protected _tracks: Track[] = [];
  /** The total duration of all tracks */
  protected _totalDuration: number = 0;
  /** The total number of (open) tracks */
  protected _countTracks: number = 0;
  /** Dynamically generated canvas to draw on */
  protected _canvas: HTMLCanvasElement | null = null;
  /** Dynamically generated scroll container. */
  protected _scrollContainer: HTMLElement | null = null;
  /** Dynamically generated virtual scroll content. (while canvas has no real size, this element is used to create virtual scroll on the parent element). */
  protected _scrollContent: HTMLElement | null = null;
  // Current zoom level.
  protected _currentZoom = 1;
  // Selection area.
  protected _selectionArea: Area | null = null;
  /** Current time for the cursor. */
  protected _currentTime: number = 0;

  init(container: HTMLElement, tracks: Track[], config: TimelineConfig) {
    this._tracks = tracks;

    // Create the virtual scroller (used to only render what visible on screen)
    this._scrollContainer = document.createElement('div');
    this._scrollContent = document.createElement('div');

    // Those styles are hardcoded and required for the proper scrolling.
    this._scrollContainer.style.cssText =
      'overflow: scroll;' +
      'position: absolute;' +
      'width:  100%;' +
      'height:  100%;scrollbar-color: #1867c0 #eeeeee';
    this._scrollContent.style.width = this._scrollContent.style.height = '100%';
    this._scrollContainer.appendChild(this._scrollContent);
    container.appendChild(this._scrollContainer);

    // Create canvas
    this._canvas = document.createElement('canvas');
    this._canvas.height = 0;
    this._canvas.width = 0;
    this._canvas.style.cssText =
      'image-rendering: -moz-crisp-edges;' +
      'image-rendering: -webkit-crisp-edges;' +
      'image-rendering: pixelated;' +
      'image-rendering: crisp-edges;' +
      'user-select: none;' +
      '-webkit-user-select: none;' +
      '-khtml-user-select: none;' +
      '-moz-user-select: none;' +
      '-o-user-select: none;' +
      'user-select: none;' +
      'touch-action: none;' +
      // "position: absolute;" +
      '-webkit-user-drag: none;' +
      '-khtml-user-drag: none;' +
      '-moz-user-drag: none;' +
      '-o-user-drag: none;' +
      'user-drag: none;' +
      'padding: inherit' +
      'margin-top: 0.5px' +
      'height: 100%';
    container.appendChild(this._canvas);

    // Calculate current browser scrollbar size and add offset for the canvas
    const scrollBarWidth = this._scrollContainer.offsetWidth - this._scrollContent.clientWidth;
    this._canvas.style.width = this._canvas.style.height =
      'calc(100% - ' + (scrollBarWidth || 17) + 'px)';

    super.init(this._canvas, tracks, config);
  }

  destroy() {
    super.destroy();
    this._scrollContainer = null;
    this._scrollContent = null;
    this._canvas = null;
  }

  /**
   * Update the tracks: this will generate a new rescale / rendering.
   */
  setTracks = (tracks: Track[]) => {
    this._tracks = tracks;
    this._totalDuration = 0;
    const countTracks = (tracks: Track[]): number =>
      tracks.reduce((count, track) => {
        this._totalDuration = Math.max(
          this._totalDuration,
          ...track.keyframes.map((keyframe) => keyframe.end),
        );
        return count + 1 + (track.open ? countTracks(track.children) : 0);
      }, 0);
    this._countTracks = countTracks(this._tracks);
    this.rescale(this._countTracks);
    this.render();
  };

  getTracks(): Track[] {
    return this._tracks;
  }

  getDuration(): number {
    return this._totalDuration;
  }

  rescale(countTracks?: number) {
    this._updateCanvasScale();
    if (this._scrollContent) {
      let changed = false;

      // Add more space vertically if needed.
      if (countTracks !== undefined) {
        this._scrollContent.style.minHeight = `${countTracks * this._config.trackHeight + this._config.headerHeight}px`;
        changed = true;
      }

      // Add more space horizontally if needed.
      const totalSize = this.valToPx(this._totalDuration);
      const currentSize = Number(this._scrollContent.style.minWidth?.replace('px', ''));
      if (totalSize != currentSize) {
        this._scrollContent.style.minWidth = `${totalSize}px`;
        changed = true;
      }
      if (changed) {
        this._updateCanvasScale();
      }
    }
  }

  render() {
    if (this._ctx) {
      this._ctx.save();
      // Take consideration for scrollY position: translating the canvas makes it easier for later calculation.
      this._ctx.translate(0.5, -(this?._scrollContainer?.scrollTop ?? 0.5));
      this._renderBackground();
      this._renderTracks();
      this._renderTimeScale(); // weird but needs to be after tracks to draw over it
      this._renderKeyframe();
      this._renderSelectionArea();
      this._renderTimeCursor();
      this._ctx.restore();
    }
  }

  /** Applies a set of handlers to TimelineItem matching the given position. */
  applyAtPosition(
    pos: Point | Area,
    onHandle: TimelineItemCallback = null,
    onKeyFrame: TimelineItemCallback = null,
    onResizer: TimelineItemCallback = null,
    onTrack: TimelineItemCallback = null,
    tracks: Track[] = this._tracks,
    counter: number = 0,
  ): number {
    if (this._ctx) {
      this._scrollContainer!.style.cursor = 'default';
      const left = this._scrollContainer!.scrollLeft;
      const right = left + this._ctx.canvas.clientWidth;
      const trackHeight = this._config.trackHeight;

      const _innerApplyAtPosition = (
        pos: Point | Area,
        onHandle: TimelineItemCallback = null,
        onKeyFrame: TimelineItemCallback = null,
        onResizer: TimelineItemCallback = null,
        onTrack: TimelineItemCallback = null,
        tracks: Track[] = this._tracks,
        counter: number = 0,
      ) => {
        for (const track of tracks) {
          const trackTop = counter * this._config.trackHeight + this._config.headerHeight;
          const trackBottom = trackTop + trackHeight;

          const area: Area = { x1: left, y1: trackTop, x2: right, y2: trackBottom };
          let isTrackAtPosition = TimelineUtils.isIntersect(pos, area);

          for (const keyframe of track.keyframes) {
            const handleLeft = this.valToPxPosition(keyframe.start) - 6;
            const handleRight = handleLeft + 12;
            const keyframeLeft = handleRight;
            const keyframeRight = this.valToPxPosition(keyframe.end) - 9;
            const resizerLeft = keyframeRight;
            const resizerRight = resizerLeft + 6;
            const keyframeTop = trackTop + 5;
            const keyframeBottom = keyframeTop + trackHeight - 10;

            const isHandleAtPosition = TimelineUtils.isIntersect(pos, {
              x1: handleLeft,
              x2: handleRight,
              y1: keyframeTop,
              y2: keyframeBottom,
            });
            if (isHandleAtPosition) this._scrollContainer!.style.cursor = 'pointer';
            if (onHandle) onHandle(keyframe as unknown as TimelineItem, isHandleAtPosition, track);

            const isKeyFrameAtPosition = TimelineUtils.isIntersect(pos, {
              x1: keyframeLeft,
              x2: keyframeRight,
              y1: keyframeTop,
              y2: keyframeBottom,
            });
            // if (isKeyFrameAtPosition) this._scrollContainer!.style.cursor = "ew-resize";
            if (isKeyFrameAtPosition) this._scrollContainer!.style.cursor = 'move';
            if (onKeyFrame)
              onKeyFrame(keyframe as unknown as TimelineItem, isKeyFrameAtPosition, track);

            const isResizerAtPosition = TimelineUtils.isIntersect(pos, {
              x1: resizerLeft,
              x2: resizerRight,
              y1: keyframeTop,
              y2: keyframeBottom,
            });
            if (isResizerAtPosition) this._scrollContainer!.style.cursor = 'col-resize';
            if (onResizer)
              onResizer(keyframe as unknown as TimelineItem, isResizerAtPosition, track);

            isTrackAtPosition =
              isTrackAtPosition &&
              !(isHandleAtPosition || isKeyFrameAtPosition || isResizerAtPosition);
          }
          track.keyframes.sort((left, right) => left.start - right.start);

          if (onTrack) onTrack(track as unknown as TimelineItem, isTrackAtPosition, track);

          if (track.children.length && track.open) {
            counter = _innerApplyAtPosition(
              pos,
              onHandle,
              onKeyFrame,
              onResizer,
              onTrack,
              track.children,
              counter + 1,
            );
          } else {
            counter++;
          }
        }
        return counter;
      };

      return _innerApplyAtPosition(pos, onHandle, onKeyFrame, onResizer, onTrack, tracks, counter);
    }
    return counter;
  }

  // ###########################################
  // Rendering helpers
  // ###########################################

  /** Renders the canvas background. */
  private _renderBackground(): void {
    if (this._ctx) {
      // Transparent background.
      this._ctx.clearRect(0, 0, this._ctx.canvas.width, this._ctx.canvas.height);
    }
  }

  /** Renders timescale: the time scale in the canvas header and the on-tracks time-step marks. */
  private _renderTimeScale() {
    if (this._ctx) {
      const screenWidth = this._ctx.canvas.clientWidth - this._config.leftMargin;
      const screenHeight = this._scrollContainer!.scrollTop + this._ctx.canvas.clientHeight;
      // Compute the time on screen depending on current zoom / scroll.
      let startTime = this.pxToVal(this._scrollContainer!.scrollLeft);
      let endTime = this.pxToVal(this._scrollContainer!.scrollLeft + screenWidth);
      const screenDuration = endTime - startTime;

      // Find the nearest 'beautiful' step for a gauge.
      // 'beautiful' step should be dividable by 1/2/5/10!
      const stepDuration = TimelineUtils.findGoodStepSize(
        screenDuration / (screenWidth / this._config.stepWidth),
      );

      // Find beautiful start/end point:
      // that means depending on the current scroll/zoom, we try to start/end with a rounded step value.
      startTime = Math.floor(startTime / stepDuration) * stepDuration;
      endTime = Math.ceil(endTime / stepDuration) * stepDuration + stepDuration;

      // Compute the intermediary steps.
      let subStepDuration = 0;
      if (this._config.stepSubWidth) {
        subStepDuration = TimelineUtils.findGoodStepSize(
          screenDuration / (screenWidth / this._config.stepSubWidth),
        );
      }

      this._ctx.save();
      let lastTextStart = 0;
      const headerHeight = this._config.headerHeight;
      const tickHeight = headerHeight / 2;
      const smallTickHeight = headerHeight / 1.3;
      for (let ms = startTime; ms <= endTime; ms += stepDuration) {
        const pxPosition = this.valToPxPosition(ms);

        this._ctx.setLineDash([4]);
        this._ctx.lineWidth = 1;
        // Draw a step in header.
        this._ctx.strokeStyle = '#D5D5D5';
        this.drawLine(pxPosition, tickHeight, pxPosition, headerHeight);
        // Draw the step in the tracks.
        this._ctx.strokeStyle = '#737070';
        this.drawLine(pxPosition, headerHeight, pxPosition, screenHeight);

        const text = TimelineUtils.formatTime(ms);
        this._ctx.fillStyle = '#D5D5D5';
        this._ctx.font = '11px sans-serif';
        const textSize = this._ctx.measureText(text);

        const textX = pxPosition - textSize.width / 2;
        // Skip text render if there is no space for it.
        if (isNaN(lastTextStart) || lastTextStart <= textX) {
          lastTextStart = textX + textSize.width;
          this._ctx.fillText(text, textX, 10);
        }

        // Draw sub steps
        if (subStepDuration) {
          for (let x = ms + subStepDuration; x < ms + stepDuration; x += subStepDuration) {
            const nextPosition = this.valToPxPosition(x);
            this._ctx.setLineDash([]);
            this._ctx.lineWidth = 1;
            this.drawLine(nextPosition, smallTickHeight, nextPosition, headerHeight);
          }
        }
      }

      this._ctx.restore();
    }
  }

  /** Renders timeline rows. */
  private _renderTracks() {
    if (this._ctx) {
      const from = 0;
      const to = from + this._ctx.canvas.clientWidth;
      const top = this._scrollContainer!.scrollTop;
      const trackHeight = this._config.trackHeight;
      const headerHeight = this._config.headerHeight;

      const firstVisibleTrack = Math.min(this._countTracks, Math.ceil(top / trackHeight));
      const lastVisibleTrack = Math.min(
        this._countTracks,
        firstVisibleTrack + Math.ceil(this._ctx.canvas.clientHeight / trackHeight),
      );

      this._ctx.save();
      this._ctx.lineWidth = 1;
      this._ctx.strokeStyle = '#737070';
      this.drawLine(0, headerHeight, this._ctx.canvas.clientWidth, headerHeight);
      for (let i = firstVisibleTrack; i <= lastVisibleTrack; i++) {
        const trackPosY = i * trackHeight + this._config.headerHeight;
        this.drawLine(from, trackPosY + trackHeight, to, trackPosY + trackHeight);
      }
      this._ctx.restore();
    }
  }

  /** Renders keyframes recursively */
  private _renderKeyframe(tracks: Track[] = this._tracks, counter = 0): number {
    if (this._ctx) {
      const screenLeft = 0; //this._scrollContainer!.scrollLeft;
      const screenRight = screenLeft + this._ctx.canvas.clientWidth;
      const screenTop = this._scrollContainer!.scrollTop;
      const screenBottom = screenTop + this._ctx.canvas.clientHeight;
      const trackHeight = this._config.trackHeight;

      this._ctx.save();
      for (const track of tracks) {
        // Determine track vertical position: stop rendering when canvas is full.
        const trackPosY = counter * trackHeight + this._config.headerHeight;
        // if (trackPosY + trackHeight < screenTop) continue;
        // if (trackPosY > screenBottom) break;

        // Draw the children track for groups.
        if (track.children.length && track.open) {
          counter = this._renderKeyframe(track.children, counter + 1);
        } else {
          counter++;
        }

        // Draw the keyframes
        for (const keyframe of track.keyframes) {
          const x = this.valToPxPosition(keyframe.start);
          const h = trackHeight - 10;
          const y = trackPosY + 10 / 2;
          const w = this.valToPx(keyframe.end - keyframe.start);

          // Do not draw keyframes outside the screen.
          if (x + w < screenLeft) continue;
          if (x > screenRight) break;

          if (track.device) {
            this.drawSimpleKeyframe(keyframe as unknown as TimelineItem, x, y, w, h);
          } else {
            this.drawGroupKeyframe(keyframe as unknown as TimelineItem, x, y, w, h);
          }
          if (track.open) {
            this.drawGroupKeyframeMarker(
              keyframe as unknown as TimelineItem,
              x,
              y + trackHeight,
              track.children.length * trackHeight,
            );
          }
          if (track.id === 0) {
            this.drawGroupKeyframeMarker(
              keyframe as unknown as TimelineItem,
              x,
              y + trackHeight,
              screenBottom,
            );
          }
          this.drawKeyframeResizer(keyframe as unknown as TimelineItem);
        }
      }
      this._ctx.restore();
    }

    return counter;
  }

  /** Renders timeline rows. */
  private _renderTimeCursor() {
    if (this._ctx) {
      const cursorPosition = this.valToPxPosition(this._currentTime);
      const screenBottom = this._scrollContainer!.scrollTop + this._ctx.canvas.clientHeight;
      this.drawTimeCursor(cursorPosition, screenBottom);
    }
  }

  /** Renders selection area. */
  private _renderSelectionArea = () => {
    if (this._ctx && this._selectionArea) {
      this.drawSelectionArea(this._selectionArea);
    }
  };

  // ###########################################
  // Converters
  // ###########################################

  /** Converts screen pixel to duration. */
  pxToVal(px: number): number {
    const steps = this._config.stepDuration * this._currentZoom || 1;
    return Math.round((px / this._config.stepWidth) * steps);
  }

  /** Convert area value to global canvas pixel coordinates. */
  valToPx = (val: number): number => {
    const steps = (this._config.stepDuration || 0) * this._currentZoom || 1;
    return Math.round(val * (this._config.stepWidth / steps));
  };

  /** Convert value to local screen component coordinates. */
  valToPxPosition(val: number): number {
    return Math.round(
      this.valToPx(val) - this._scrollContainer!.scrollLeft + this._config.leftMargin,
    );
  }

  /** Convert value to local screen component coordinates. */
  pxPositionToVal(px: number): number {
    return Math.round(
      this.pxToVal(this._scrollContainer!.scrollLeft + px - this._config.leftMargin),
    );
  }

  // ###########################################
  // Event handling
  // ###########################################

  protected _getMousePositionOnCanvas(e: TouchEvent | MouseEvent): Point {
    let clientX: number;
    let clientY: number;
    if (e instanceof TouchEvent && e.changedTouches.length > 0) {
      const touch = e.changedTouches[0];
      clientX = touch.clientX;
      clientY = touch.clientY;
    } else if (e instanceof MouseEvent) {
      clientX = e.clientX;
      clientY = e.clientY;
    } else {
      throw new Error('Unsupported event type');
    }

    const rect = this._ctx!.canvas.getBoundingClientRect();
    const x = clientX - (rect?.left ?? 0);
    const y = clientY - (rect?.top ?? 0) + (this._scrollContainer?.scrollTop ?? 0);

    return { x, y };
  }

  onZoom(event: WheelEvent) {
    if (this._ctx) {
      let { x } = this._getMousePositionOnCanvas(event);
      x = Math.max(0, x || 0);

      const deltaSpeed = TimelineUtils.getDistance(this._ctx.canvas.clientWidth / 2, x) * 0.2;
      x = x + deltaSpeed;
      const diff = this._ctx.canvas.clientWidth / x;
      const val = this.pxPositionToVal(x);
      const zoom = TimelineUtils.sign(event.deltaY) * this._currentZoom * this._config.zoomSpeed;
      this._setZoom(this._currentZoom + zoom);
      // Get only after zoom is set
      const zoomCenter = this.valToPx(val);
      this._scrollContainer!.scrollLeft = Math.max(
        Math.round(zoomCenter - this._ctx.canvas.clientWidth / diff),
        0,
      );

      this.rescale();
      this.render();
    }
  }

  // ###########################################

  /**
   * Set direct zoom value.
   * @param zoom zoom value to set. percent 0-1 and etc.
   * @param min min zoom.
   * @param max max zoom.
   * @return normalized value.
   */
  private _setZoom(
    zoom: number,
    min: number = this._config.zoomMin,
    max: number = this._config.zoomMax,
  ) {
    zoom = TimelineUtils.clampValue(zoom, min, max);
    zoom = zoom || 1;
    this._currentZoom = zoom;
  }

  private _updateCanvasScale(): boolean {
    if (this._ctx) {
      let changed = false;
      const width = this._ctx.canvas.clientWidth * this._config.scaleFactor;
      const height = this._ctx.canvas.clientHeight * this._config.scaleFactor;
      if (Math.floor(width) != Math.floor(this._ctx.canvas.width)) {
        this._ctx.canvas.width = width;
        changed = true;
      }

      if (Math.floor(height) != Math.floor(this._ctx.canvas.height)) {
        this._ctx.canvas.height = height;
        changed = true;
      }

      if (changed) {
        this._ctx.scale(this._config.scaleFactor, this._config.scaleFactor);
      }
      return changed;
    }
    return false;
  }
}
