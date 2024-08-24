// Nested to Flat Conversion
import { Keyframe } from '@/types/animation';
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
export function useFlatToNested<Flat extends FlatGroup, Nested extends NestedGroup>(
  groups: Record<GroupId, Flat>,
  keyframes: Record<GroupId, Keyframe[]> = {},
): Nested[] {
  const nestedGroups: Record<GroupId, Nested> = {};

  // Create equivalent NestedGroup for all FlatGroup.

  Object.keys(groups).forEach((id: GroupId) => {
    nestedGroups[id] = {
      ...groups[id],
      children: [] as Nested[],
      level: 0,
      keyframes: [...(keyframes[id] ?? [])],
    } as unknown as Nested;
  });

  // Helper function to build the nested structure.
  function traverse(groupId: GroupId, level: number = 0): Nested | null {
    const nestedGroup: Nested = nestedGroups[groupId];
    if (!nestedGroup) {
      return null;
    }

    nestedGroup['level'] = level;

    // Recursively attach (and sort) child groups.
    nestedGroup.children = groups[groupId].children
      .map((childId) => traverse(childId, level + 1))
      .filter((group) => group)
      .sort((left, right) => left!.order - right!.order) as Nested[];

    return nestedGroup;
  }

  // Step 3: Find all root groups (those that are not a child of any other group)
  const roots: Nested[] = [];
  const childIds = new Set<GroupId>(Object.values(groups).flatMap((group) => group.children));

  Object.values(groups).reduce((roots, group) => {
    if (!childIds.has(group.id)) {
      const root = traverse(group.id);
      if (root) {
        roots.push(root);
      }
    }
    return roots;
  }, roots);

  return roots.sort((left, right) => left.order - right.order) as Nested[];
}
