import { Entity } from '@/types/core';
import { DeviceId } from '@/types/devices';

export declare type GroupId = Branded<number, 'GroupId'>;

export declare type Enrichment = {
  [key: string]: unknown;
};

export declare type FlatGroup = Entity<GroupId> & {
  device?: DeviceId;
  order: number;
  children: GroupId[];
} & Enrichment;

export declare type NestedGroup = FlatGroup & {
  children: NestedGroup[];
  level: number;
};
