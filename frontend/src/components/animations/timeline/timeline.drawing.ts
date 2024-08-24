import { Area, TimelineConfig, TimelineItem, Track } from '@/types/timeline';

export default class TimelineDrawing {
  /** Rendering context */
  protected _ctx: CanvasRenderingContext2D | null = null;
  /** Customizables */
  protected _config!: TimelineConfig;

  init(canvas: HTMLCanvasElement, _: Track[], config: TimelineConfig) {
    this._config = config;
    // Creates the drawing context from the canvas.
    this._ctx = canvas.getContext('2d') as CanvasRenderingContext2D;
    // Align point/pixel in canvas (https://stackoverflow.com/questions/4261090/html5-canvas-and-anti-aliasing)
    this._ctx.translate(0.5, 0.5);
  }

  destroy() {
    this._ctx = null;
  }

  /** Draws a line from (x1,y1) to (x2,y2). */
  drawLine(x1: number, y1: number, x2: number, y2: number): void {
    if (this._ctx) {
      this._ctx.beginPath();
      this._ctx.moveTo(x1, y1);
      this._ctx.lineTo(x2, y2);
      this._ctx.stroke();
    }
  }

  /** Draws the keyframe for a group */
  drawGroupKeyframe(keyframe: TimelineItem, x: number, y: number, w: number, h: number): void {
    if (this._ctx) {
      this._drawKeyframeDuration(keyframe, x, y, w, h);
      this._drawRectHandle(keyframe, x, y, h);
    }
  }

  /** Draws the keyframe for a group */
  drawSimpleKeyframe(keyframe: TimelineItem, x: number, y: number, w: number, h: number): void {
    this._drawKeyframeDuration(keyframe, x, y, w, h);
    this._drawDiamondHandle(keyframe, x, y + h / 2, 12, 12);
  }

  /** Draws the group marker (a line indication for group keyframe when a group is open */
  drawGroupKeyframeMarker(
    keyframe: TimelineItem,
    x: number,
    y: number,
    groupHeight: number = 0,
  ): void {
    if (this._ctx && groupHeight) {
      this._ctx.save();
      this._ctx.strokeStyle = this._config.colorPrimary;
      this._ctx.lineWidth = 3;
      this._ctx.setLineDash([3]);
      this.drawLine(x, y - 10, x, y + groupHeight - 5);
      this._ctx.restore();
    }
  }

  /** Draws the keyframe resizer anchor. */
  drawKeyframeResizer(keyframe: TimelineItem) {}

  /** Draw the time cursor */
  drawTimeCursor(x: number, bottom: number): void {
    if (this._ctx) {
      this._ctx.save();
      this._ctx.strokeStyle = this._config.colorTimeCursor;
      this._ctx.lineWidth = 2;
      this.drawLine(x, 15, x, bottom);
      this._ctx.lineWidth = 6;
      this.drawLine(x, 15, x, 30);
      this._ctx.restore();
    }
  }

  /** Draw selection area */
  drawSelectionArea(selectionArea: Area): void {
    if (this._ctx) {
      this._ctx.save();
      this._ctx.beginPath();
      this._ctx.strokeStyle = this._config.colorHandle;
      this._ctx.lineWidth = 1;
      this._ctx.setLineDash([4]);
      this._ctx.rect(
        selectionArea.x1,
        selectionArea.y1,
        selectionArea.x2 - selectionArea.x1,
        selectionArea.y2 - selectionArea.y1,
      );
      this._ctx.stroke();
      this._ctx.restore();
    }
  }

  // ## HELPERS ##

  private _drawKeyframeDuration(
    keyframe: TimelineItem,
    x: number,
    y: number,
    w: number,
    h: number,
  ): void {
    if (this._ctx) {
      this._ctx.save();
      this._ctx.fillStyle = keyframe.selected
        ? this._config.colorPrimaryLighten
        : this._config.colorPrimary;
      this._ctx.beginPath();
      this._ctx.fillRect(x, y, w, h);
      this._ctx.fill();
      this._ctx.restore();
    }
  }

  private _drawDiamondHandle(
    keyframe: TimelineItem,
    x: number,
    y: number,
    w: number,
    h: number,
  ): void {
    if (this._ctx) {
      this._ctx.save();

      this._ctx.beginPath();
      this._ctx.translate(x, y);
      this._ctx.rotate((45 * Math.PI) / 180);
      this._ctx.fillStyle = keyframe.hovered ? this._config.colorSelectedHandle : '#383838';
      this._ctx.fillRect(-w / 2, -h / 2, w, h);
      this._ctx.stroke();

      const borderSize = 2; // border
      this._ctx.fillStyle =
        keyframe.hovered || keyframe.selected ? this._config.colorSelectedHandle : 'white';
      this._ctx.translate(borderSize, borderSize);
      this._ctx.fillRect(-w / 2, -h / 2, w - borderSize * 2, h - borderSize * 2);
      this._ctx.stroke();
      this._ctx.restore();
    }
  }

  private _drawRectHandle(keyframe: TimelineItem, x: number, y: number, h: number) {
    if (this._ctx) {
      this._ctx.save();
      this._ctx.beginPath();
      const borderSize = 2;
      this._ctx.lineWidth = borderSize;
      this._ctx.strokeStyle = keyframe.hovered ? 'yellow' : '#383838';
      this._ctx.fillStyle = keyframe.hovered || keyframe.selected ? 'yellow' : 'white';
      this._ctx.rect(x - 3, y - borderSize, 6, h + borderSize * 2);
      this._ctx.fill();
      this._ctx.stroke();
      this._ctx.restore();
    }
  }

  setConfig(config: TimelineConfig) {
    this._config = config;
  }
}
