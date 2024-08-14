// export declare type TrackId = Branded<string, 'TrackId'>;
//
// export declare interface Track {
//   id: TrackId;
//
//   name?: string;
//   group?: Group;
//   device?: Device;
//   keyframes: Keyframe[];
//   children: Track[];
//   keyframes: Keyframe[];
//   level?: number;
//   open?: boolean;
//   disabled?: boolean;
// }

export declare type KeyframeId = Branded<number, 'KeyframeId'>;

export declare interface Keyframe {
  id: KeyframeId;
}
