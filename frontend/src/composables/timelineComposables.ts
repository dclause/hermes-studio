import { TimelineConfig } from '@/types/timeline';

export enum Easing {
  /// https://easings.net/#easeInBack
  BackIn = 'BackIn',
  /// https://easings.net/#easeInOutBack
  BackInOut = 'BackInOut',
  /// https://easings.net/#easeOutBack
  BackOut = 'BackOut',
  /// https://easings.net/#easeInBounce
  BounceIn = 'BounceIn',
  /// https://easings.net/#easeInOutBounce
  BounceInOut = 'BounceInOut',
  /// https://easings.net/#easeOutBounce
  BounceOut = 'BounceOut',
  /// https://easings.net/#easeInCirc
  CircIn = 'CircIn',
  /// https://easings.net/#easeInOutCirc
  CircInOut = 'CircInOut',
  /// https://easings.net/#easeOutCirc
  CircOut = 'CircOut',
  /// https://easings.net/#easeInCubic
  CubicIn = 'CubicIn',
  /// https://easings.net/#easeInOutCubic
  CubicInOut = 'CubicInOut',
  /// https://easings.net/#easeOutCubic
  CubicOut = 'CubicOut',
  /// https://easings.net/#easeInElastic
  ElasticIn = 'ElasticIn',
  /// https://easings.net/#easeInOutElastic
  ElasticInOut = 'ElasticInOut',
  /// https://easings.net/#easeOutElastic
  ElasticOut = 'ElasticOut',
  /// https://easings.net/#easeInExpo
  ExpoIn = 'ExpoIn',
  /// https://easings.net/#easeInOutExpo
  ExpoInOut = 'ExpoInOut',
  /// https://easings.net/#easeOutExpo
  ExpoOut = 'ExpoOut',
  // Applies no transformation (default).
  Linear = 'Linear',
  /// https://easings.net/#easeInQuad
  QuadIn = 'QuadIn',
  /// https://easings.net/#easeInOutQuad
  QuadInOut = 'QuadInOut',
  /// https://easings.net/#easeOutQuad
  QuadOut = 'QuadOut',
  /// https://easings.net/#easeInQuart
  QuartIn = 'QuartIn',
  /// https://easings.net/#easeInOutQuart
  QuartInOut = 'QuartInOut',
  /// https://easings.net/#easeOutQuart
  QuartOut = 'QuartOut',
  /// https://easings.net/#easeInQuint
  QuintIn = 'QuintIn',
  /// https://easings.net/#easeInOutQuint
  QuintInOut = 'QuintInOut',
  /// https://easings.net/#easeOutQuint
  QuintOut = 'QuintOut',
  // A linear easing that goes from 1.0 to 0.0.
  Reverse = 'Reverse',
  // A linear easing that goes from 0.0 to 1.0 and back to 0.0. That might be used in combination with other easing functions.
  RoundTrip = 'RoundTrip',
  /// https://easings.net/#easeInSine
  SineIn = 'SineIn',
  /// https://easings.net/#easeInOutSine
  SineInOut = 'SineInOut',
  /// https://easings.net/#easeOutSine
  SineOut = 'SineOut',
}

// @todo move this inside vuetify ?
export const timelineConfig: TimelineConfig = {
  // styling options.
  colorPrimary: '#3191ef', //"#1867c0",
  colorSecondary: '#48a9a6',
  colorHandle: 'white',
  colorTimeCursor: 'orange',
  colorSelectedHandle: 'yellow',
  // render options.
  scaleFactor: 3, // Scale factor (ie the ratio of pixels in the ctx drawing width as compared to the canvas width)
  headerHeight: 36, // Height of header in px (better be divisible by 2 and 3)
  trackHeight: 24, // Height of track in px.
  leftMargin: 16, // Left margin size in pixels.
  stepDuration: 1000, // Step time in ms.
  stepWidth: 120, // Step size in pixels.
  stepSubWidth: 30, // Step subdivision size in pixels.
  zoomMin: 0.05,
  zoomMax: 15,
  zoomSpeed: 0.1,
};

export class Timeline {}

let timeline = new Timeline();

export function useTimeline() {
  return {
    timeline,
    config: timelineConfig,
    reset: () => {
      timeline = new Timeline();
    },
  };
}
