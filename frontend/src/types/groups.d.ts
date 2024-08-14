import { Entity } from '@/types/core';

export declare type GroupId = Branded<number, 'GroupId'>;

export declare interface FlatGroup extends Entity<GroupId> {
  device?: number;
  order: number;
  children: GroupId[];
}

export declare interface NestedGroup extends FlatGroup {
  children: NestedGroup[];
  level: number;
}
