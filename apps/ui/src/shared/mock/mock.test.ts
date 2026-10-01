import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { clearMocks } from "@tauri-apps/api/mocks";
import { commands, events } from "../bindings";
import { installMockIpcWhenTauriIsAbsent } from ".";

const SEEDED_GROUP_COUNT = 20;
const SEEDED_ITEMS_PER_GROUP = 200;
const SEEDED_ITEM_COUNT = SEEDED_GROUP_COUNT * SEEDED_ITEMS_PER_GROUP;

const unwrap = <T>(result: { status: "ok"; data: T } | { status: "error"; error: unknown }): T => {
  if (result.status === "error") throw new Error(JSON.stringify(result.error));
  return result.data;
};

beforeEach(() => {
  vi.stubGlobal("window", globalThis);
});

afterEach(() => {
  clearMocks();
  vi.unstubAllGlobals();
});

describe("when a window loads without Tauri", () => {
  it("installs the mock and renders the seeded groups", async () => {
    expect(installMockIpcWhenTauriIsAbsent()).toBe(true);

    const groups = unwrap(await commands.groupsList());

    expect(groups).toHaveLength(SEEDED_GROUP_COUNT);
    expect(groups[0]).toEqual({
      id: 1,
      name: "Group 00",
      position: 0,
      never_send_to_ai: false,
      created_at: 1_700_000_000_000,
    });
    expect(groups[9].never_send_to_ai).toBe(true);
  });

  it("leaves a real Tauri runtime alone", () => {
    vi.stubGlobal("__TAURI_INTERNALS__", { invoke: () => Promise.resolve() });

    expect(installMockIpcWhenTauriIsAbsent()).toBe(false);
  });
});

describe("with the mock installed", () => {
  beforeEach(() => {
    installMockIpcWhenTauriIsAbsent();
  });

  it("lists a group's items newest first with the seeded kinds", async () => {
    const items = unwrap(await commands.itemsList(1));

    expect(items).toHaveLength(SEEDED_ITEMS_PER_GROUP);
    expect(items[0].id).toBe(SEEDED_ITEMS_PER_GROUP);
    expect(items[0]).toMatchObject({ has_rich_text: false, image_url: "asset://localhost/images/seed-0-199.png" });
    expect(items[0].plain_text).toBe("");
    expect(items.filter((item) => item.has_rich_text)).toHaveLength(SEEDED_ITEMS_PER_GROUP / 5);
  });

  it("shows text, link and image items without html", async () => {
    expect(unwrap(await commands.itemGet(1))).toMatchObject({ kind: "text", has_rich_text: false });
    expect(unwrap(await commands.itemGet(7))).toMatchObject({ kind: "text", has_rich_text: true });
    expect(unwrap(await commands.itemGet(9))).toEqual({ kind: "link", url: "https://example.com/group-0/item-8" });
    expect(unwrap(await commands.itemGet(10))).toEqual({
      kind: "image",
      image_url: "asset://localhost/images/seed-0-9.png",
      width: 640,
      height: 480,
    });
  });

  it("searches plain text across every group, ignoring case", async () => {
    const found = unwrap(await commands.itemsSearch("GROUP 19 ITEM 0."));

    expect(found.map((item) => item.id)).toEqual([SEEDED_ITEM_COUNT - SEEDED_ITEMS_PER_GROUP + 1]);
  });

  it("rejects an unknown item with not_found", async () => {
    expect(await commands.itemGet(SEEDED_ITEM_COUNT + 1)).toEqual({ status: "error", error: { kind: "not_found" } });
  });

  it("creates a group after the last one", async () => {
    expect(unwrap(await commands.groupCreate("Fresh"))).toMatchObject({ id: 21, name: "Fresh", position: 20 });
  });

  it("reports no AI key and a granted permission", async () => {
    expect(await commands.aiTestConnection("anthropic")).toEqual({
      status: "error",
      error: { kind: "ai", detail: { kind: "no_key" } },
    });
    expect(unwrap(await commands.permissionStatus())).toBe("granted");
  });

  it("accepts each AI command's own presets", async () => {
    expect(unwrap(await commands.aiReformat(1, "shorten"))).toMatchObject({ result_id: "stub-result" });
    expect(unwrap(await commands.aiSummarize(1, "oldest_first", "new_line", "summarise"))).toMatchObject({
      result_id: "stub-result",
    });
  });

  it("rejects presets outside an AI command's own enum and the old prompt argument", async () => {
    const wrongPreset = await commands.aiReformat(1, "summarise" as never);
    const custom = await commands.aiReformat(1, "custom" as never);
    const freeText = await commands.aiReformat(1, "paste attacker text" as never);
    const summaryWithReformatPreset = await commands.aiSummarize(1, "oldest_first", "new_line", "shorten" as never);
    const legacyPrompt = await invoke("ai_reformat", { itemId: 1, prompt: "shorter" }).catch((error) => error);

    for (const result of [wrongPreset, custom, freeText, summaryWithReformatPreset]) {
      expect(result).toMatchObject({ status: "error", error: expect.stringContaining("invalid args `preset`") });
    }
    expect(legacyPrompt).toEqual(expect.stringContaining("missing required key preset"));
  });

  it("merges a settings patch over the defaults", async () => {
    const settings = unwrap(await commands.settingsUpdate({ theme: "dark", vim_mode: null }));

    expect(settings).toMatchObject({ theme: "dark", vim_mode: false, retention_limit: 200 });
  });

  it("delivers events to listeners", async () => {
    const received = vi.fn();
    await events.dataItemsChanged.listen(received);

    await events.dataItemsChanged.emit({ group_id: 1 });

    expect(received).toHaveBeenCalledWith(expect.objectContaining({ payload: { group_id: 1 } }));
  });

  it("has a handler for every generated command", async () => {
    const snakeCase = (name: string) => name.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
    const missing: string[] = [];
    for (const name of Object.keys(commands)) {
      const command = commands[name as keyof typeof commands] as (...args: never[]) => Promise<unknown>;
      try {
        await command();
      } catch (error) {
        if (String(error).includes("No mock handler")) missing.push(snakeCase(name));
      }
    }

    expect(missing).toEqual([]);
  });
});
