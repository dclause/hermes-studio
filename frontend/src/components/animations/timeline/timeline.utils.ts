import { Area, Point } from '@/types/timeline';

export default class TimelineUtils {
  /** Check if the given `pos` (point or area) intersects with the given `area`. */
  static isIntersect(pos: Point | Area, area: Area): boolean {
    if ('x' in pos && 'y' in pos) {
      // pos is of type Point
      return pos.x >= area.x1 && pos.x <= area.x2 && pos.y >= area.y1 && pos.y <= area.y2;
    } else if ('x1' in pos && 'y1' in pos && 'x2' in pos && 'y2' in pos) {
      // pos is of type Area
      pos = {
        x1: Math.min(pos.x1, pos.x2),
        x2: Math.max(pos.x1, pos.x2),
        y1: Math.min(pos.y1, pos.y2),
        y2: Math.max(pos.y1, pos.y2),
      };
      return !(pos.x2 < area.x1 || pos.x1 > area.x2 || pos.y2 < area.y1 || pos.y1 > area.y2);
    }
    return false;
  }

  /** Find beautiful step size for the header line gauge. */
  static findGoodStepSize(originalStep: number): number {
    const pow = Math.floor(Math.log10(originalStep));
    const denominators = [1, 2, 5, 10];
    let bestStep = originalStep;
    let smallestDistance = Infinity;

    for (const denominator of denominators) {
      const calculatedStep = denominator * Math.pow(10, pow);
      const distance = TimelineUtils.getDistance(originalStep, calculatedStep);

      // If the denominator is perfect, we keep it.
      if (distance === 0 || (distance <= 0.1 && pow > 0)) {
        return calculatedStep;
      }

      // Otherwise, take the closest match.
      if (distance < smallestDistance) {
        smallestDistance = distance;
        bestStep = calculatedStep;
      }
    }

    return bestStep;
  }

  /** Clamps a value in min, max bounds. */
  static clampValue(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
  }

  /** Get the distance between two points (or value) in XY coordinates. */
  static getDistance(x1: number, y1: number, x2?: number, y2?: number): number {
    if (x2 != undefined && y2 != undefined) {
      return Math.sqrt(Math.pow(x1 - x2, 2) + Math.pow(y1 - y2, 2));
    } else {
      return Math.abs(x1 - y1);
    }
  }

  /**
   * Get sign of the number. 1 or -1.
   */
  static sign(p: number): number {
    return p >= 0 ? 1 : -1;
  }

  /**
   * Format line gauge textas HH:MM:SS:mm where '00' are trimmed.
   */
  static formatTime(ms: number, isSeconds = false): string {
    // 1- Convert to seconds:
    let seconds = ms / 1000;
    if (isSeconds) {
      seconds = ms;
    }

    const year = Math.floor(seconds / (365 * 86400));
    seconds = seconds % (365 * 86400);

    const days = Math.floor(seconds / 86400);
    seconds = seconds % 86400;

    // 2- Extract hours:
    const hours = Math.floor(seconds / 3600); // 3,600 seconds in 1 hour
    seconds = seconds % 3600; // seconds remaining after extracting hours
    // 3- Extract minutes:
    const minutes = Math.floor(seconds / 60); // 60 seconds in 1 minute
    // 4- Keep only seconds not extracted to minutes:
    seconds = seconds % 60;
    let str = '';
    if (year) {
      str += year.toString().padStart(2, '0') + ':';
    }

    if (days) {
      str += days.toString().padStart(2, '0') + ':';
    }

    if (hours) {
      str += hours.toString().padStart(2, '0') + ':';
    }

    if (minutes) {
      str += hours ? minutes.toString().padStart(2, '0') : minutes + ':';
    }

    if (!isNaN(seconds)) {
      str += minutes ? seconds.toString().padStart(2, '0') : seconds;
    }

    return str;
  }

  static deepClone = <T>(previousOptions: T): T => {
    return JSON.parse(JSON.stringify(previousOptions)) as T;
  };
}
