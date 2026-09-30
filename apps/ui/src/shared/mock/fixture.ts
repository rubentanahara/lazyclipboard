import type { Group, GroupId, ItemId } from "../bindings";

const GROUP_COUNT = 20;
const ITEMS_PER_GROUP = 200;
const FIRST_CAPTURED_AT_MS = 1_700_000_000_000;
const PARAGRAPH = "The quick brown fox jumps over the lazy dog. ";
const PARAGRAPH_REPEATS = 44;
const IMAGE_WIDTH = 640;
const IMAGE_HEIGHT = 480;
const NEVER_SEND_TO_AI_EVERY = 10;
export const IMAGE_URL_PREFIX = "asset://localhost/images/";
export const IMAGE_SIZE = { width: IMAGE_WIDTH, height: IMAGE_HEIGHT };

export type FixtureItem = {
  id: ItemId;
  groupId: GroupId;
  kind: "text" | "rich_text" | "link" | "image";
  plainText: string;
  imageFile: string | null;
};

export const seededGroups = (): Group[] =>
  Array.from({ length: GROUP_COUNT }, (_, group) => ({
    id: group + 1,
    name: `Group ${String(group).padStart(2, "0")}`,
    position: group,
    never_send_to_ai: group % NEVER_SEND_TO_AI_EVERY === NEVER_SEND_TO_AI_EVERY - 1,
    created_at: FIRST_CAPTURED_AT_MS,
  }));

export const seededItems = (): FixtureItem[] =>
  Array.from({ length: GROUP_COUNT }, (_, group) =>
    Array.from({ length: ITEMS_PER_GROUP }, (_, index) => seededItem(group, index)),
  ).flat();

const seededItem = (group: number, index: number): FixtureItem => {
  const base = { id: group * ITEMS_PER_GROUP + index + 1, groupId: group + 1 };
  const body = `Group ${group} item ${index}. ${PARAGRAPH.repeat(PARAGRAPH_REPEATS)}`;
  switch (index % 10) {
    case 6:
    case 7:
      return { ...base, kind: "rich_text", plainText: body, imageFile: null };
    case 8:
      return { ...base, kind: "link", plainText: `https://example.com/group-${group}/item-${index}`, imageFile: null };
    case 9:
      return { ...base, kind: "image", plainText: "", imageFile: `seed-${group}-${index}.png` };
    default:
      return { ...base, kind: "text", plainText: body, imageFile: null };
  }
};
