export declare type GroupId = Branded<number, 'GroupId'>;

export declare interface FlatGroup {
  id: GroupId;
  name?: string;
  device?: number;
  order: number;
  children: GroupId[];
}

export declare interface NestedGroup extends FlatGroup {
  children: NestedGroup[];
  level: number;
}
