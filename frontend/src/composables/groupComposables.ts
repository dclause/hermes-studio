// Nested to Flat Conversion
import { FlatGroup, GroupId, NestedGroup } from '@/types/groups';

// Convert from nested to flat representation
export function useNestedToFlat(groups: NestedGroup[]): Record<GroupId, FlatGroup> {
  const flatMap: Record<GroupId, FlatGroup> = {};

  // Traverse and collect all groups into the map
  function traverse(nestedGroup: NestedGroup, currentOrder: number): GroupId {
    let childOrder = 0;
    const flatGroup: FlatGroup = {
      ...nestedGroup,
      order: currentOrder,
      children: nestedGroup.children.map((child: NestedGroup) => traverse(child, ++childOrder)),
    };
    flatMap[flatGroup.id] = flatGroup;

    return flatGroup.id;
  }

  let order = 0;
  for (const group of groups) {
    traverse(group, ++order);
  }
  return flatMap;
}

// Flat to Nested Conversion
export function useFlatToNested(groups: Record<GroupId, FlatGroup>): NestedGroup[] {
  const map: Record<GroupId, NestedGroup> = {};

  // Sort FlatGroups and map it to NestedGroups.
  Object.values(groups)
    .sort((left, right) => left.order - right.order)
    .forEach((group) => {
      map[group.id] = { ...group, children: [] as NestedGroup[], level: 0 };
    });

  // Helper function to build the nested structure.
  function traverse(groupId: GroupId, level: number = 1): NestedGroup | null {
    const nestedGroup: NestedGroup = map[groupId];
    const flatGroup: FlatGroup = groups[groupId];
    if (nestedGroup.level) {
      return null;
    }

    nestedGroup.children = flatGroup.children
      .map((childId) => traverse(childId, level + 1))
      .filter((group) => group) as NestedGroup[];
    nestedGroup.level = level;

    return nestedGroup;
  }

  // Loop over the groups and nest them.
  return Object.values(map)
    .map((group) => traverse(group.id))
    .filter((group) => group) as NestedGroup[];
}
