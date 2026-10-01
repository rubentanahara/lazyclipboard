import type {
  AiResult,
  CommandError,
  Group,
  GroupId,
  ItemPreview,
  ItemView,
  PlatformInfo,
  ReformatPreset,
  Settings,
  SettingsPatch,
  SummaryPreset,
} from "../bindings";
import { IMAGE_SIZE, IMAGE_URL_PREFIX, seededGroups, seededItems, type FixtureItem } from "./fixture";

type Args = Record<string, unknown>;
type Handler = (args: Args) => unknown;

const STUB_AI_RESULT: AiResult = { result_id: "stub-result", text: "Stub AI result" };
const STUB_DIAGNOSTICS_PATH = "lazyclipboard-diagnostics.zip";
const DEFAULT_SETTINGS: Settings = {
  theme: "system",
  shortcuts: { copy_to_group: "Cmd+Alt+C", paste_from_group: "Cmd+Alt+V" },
  vim_mode: false,
  launch_at_login: false,
  usage_stats: true,
  retention_limit: 200,
  auto_delete_days: null,
  ai_provider: null,
  ai_model: null,
};
const PLATFORM: PlatformInfo = { os: "macos", session: null };
const NOT_FOUND: CommandError = { kind: "not_found" };
const REFORMAT_PRESETS: readonly ReformatPreset[] = ["fix_grammar", "shorten", "make_formal"];
const SUMMARY_PRESETS: readonly SummaryPreset[] = ["summarise"];
const NO_KEY: CommandError = { kind: "ai", detail: { kind: "no_key" } };

export const createHandlers = (): Record<string, Handler> => {
  const groups = seededGroups();
  const items = seededItems();
  const newestFirst = [...items].reverse();

  const group = (id: unknown): Group => groups.find((candidate) => candidate.id === id) ?? fail(NOT_FOUND);
  const item = (id: unknown): FixtureItem => items.find((candidate) => candidate.id === id) ?? fail(NOT_FOUND);
  const created = (name: unknown): Group => {
    const [template] = groups;
    return { ...template, id: groups.length + 1, name: String(name), position: groups.length, never_send_to_ai: false };
  };
  const done = (): null => null;
  const returning = (value: unknown, find: (id: unknown) => unknown, key: string): Handler => (args) => {
    find(args[key]);
    return value;
  };
  const withPreset = (command: string, presets: readonly string[], handler: Handler): Handler => (args) => {
    if (!("preset" in args)) return fail(`invalid args \`preset\` for command \`${command}\`: command ${command} missing required key preset`);
    if (!presets.includes(String(args.preset))) {
      return fail(`invalid args \`preset\` for command \`${command}\`: unknown variant \`${String(args.preset)}\`, expected one of ${presets.join(", ")}`);
    }
    return handler(args);
  };
  const whenFound = (find: (id: unknown) => unknown, key: string): Handler => returning(null, find, key);

  return {
    groups_list: () => groups,
    group_create: ({ name }) => created(name),
    group_rename: whenFound(group, "id"),
    group_reorder: done,
    group_delete: whenFound(group, "id"),
    group_set_never_send_to_ai: whenFound(group, "id"),
    items_list: ({ groupId }) => newestFirst.filter((candidate) => candidate.groupId === groupId).map(preview),
    items_search: ({ query }) => {
      const needle = String(query).toLowerCase();
      return newestFirst.filter((candidate) => candidate.plainText.toLowerCase().includes(needle)).map(preview);
    },
    item_get: ({ id }) => view(item(id)),
    item_delete: whenFound(item, "id"),
    item_undo_delete: done,
    capture_save: ({ target }) => {
      const { kind, group_id, name } = target as { kind: string; group_id: GroupId; name: string };
      return kind === "existing" ? group(group_id) : created(name);
    },
    capture_discard: done,
    paste_item: whenFound(item, "id"),
    paste_all: whenFound(group, "groupId"),
    paste_ai_result: done,
    panel_close: done,
    ai_reformat: withPreset("ai_reformat", REFORMAT_PRESETS, returning(STUB_AI_RESULT, item, "itemId")),
    ai_summarize: withPreset("ai_summarize", SUMMARY_PRESETS, returning(STUB_AI_RESULT, group, "groupId")),
    ai_key_set: done,
    ai_key_delete: done,
    ai_key_status: () => ({ anthropic: false, openai: false, gemini: false }),
    ai_test_connection: () => fail(NO_KEY),
    settings_get: () => DEFAULT_SETTINGS,
    settings_update: ({ patch }) => applied(patch as SettingsPatch),
    shortcut_set: done,
    platform_info: () => PLATFORM,
    permission_status: () => "granted",
    permission_open_settings: done,
    window_open: done,
    diagnostics_export: () => STUB_DIAGNOSTICS_PATH,
    usage_clear: done,
  };
};

const fail = (error: CommandError | string): never => {
  throw error;
};

const imageUrl = (file: string): string => `${IMAGE_URL_PREFIX}${file}`;

const preview = (item: FixtureItem): ItemPreview => ({
  id: item.id,
  plain_text: item.plainText,
  has_rich_text: item.kind === "rich_text",
  image_url: item.imageFile === null ? null : imageUrl(item.imageFile),
});

const view = (item: FixtureItem): ItemView => {
  if (item.kind === "link") return { kind: "link", url: item.plainText };
  if (item.imageFile !== null) return { kind: "image", image_url: imageUrl(item.imageFile), ...IMAGE_SIZE };
  return { kind: "text", text: item.plainText, has_rich_text: item.kind === "rich_text" };
};

const applied = (patch: SettingsPatch): Settings => ({
  ...DEFAULT_SETTINGS,
  ...Object.fromEntries(Object.entries(patch).filter(([, value]) => value != null)),
});
