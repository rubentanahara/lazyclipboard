import { mockIPC } from "@tauri-apps/api/mocks";
import { createHandlers } from "./handlers";

export const installMockIpc = (): void => {
  const handlers = createHandlers();
  mockIPC(
    (command, args) => {
      const handler = handlers[command];
      if (handler === undefined) throw new Error(`No mock handler for command ${command}`);
      return handler((args ?? {}) as Record<string, unknown>);
    },
    { shouldMockEvents: true },
  );
};

export const installMockIpcWhenTauriIsAbsent = (): boolean => {
  const { __TAURI_INTERNALS__: internals } = window as Window & { __TAURI_INTERNALS__?: { invoke?: unknown } };
  if (typeof internals?.invoke === "function") return false;
  installMockIpc();
  return true;
};
