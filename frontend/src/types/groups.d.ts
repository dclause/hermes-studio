import { Entity } from '@/types/core';
import { DeviceId } from '@/types/devices';

export declare type GroupId = Branded<number, 'GroupId'>;

export declare interface FlatGroup extends Entity<GroupId> {
  device?: DeviceId;
  order: number;
  children: GroupId[];
}

export declare interface NestedGroup extends FlatGroup {
  children: NestedGroup[];
  level: number;
}
