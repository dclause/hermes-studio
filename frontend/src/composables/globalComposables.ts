export enum HardwareMode {
  NONE,
  VIRTUAL,
  REALTIME,
}

export function logError(error: Error) {
  console.error(error);
}
