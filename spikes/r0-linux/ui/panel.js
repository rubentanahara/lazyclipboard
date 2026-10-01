const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const query = document.getElementById("query");
const items = [...document.querySelectorAll('[role="option"]')];
const keyLog = document.getElementById("key-log");
const status = document.getElementById("status");
let selected = 0;
let pasting = false;

function select(index) {
  selected = (index + items.length) % items.length;
  items.forEach((item, position) =>
    item.setAttribute("aria-selected", String(position === selected)),
  );
}

function reset() {
  query.value = "";
  keyLog.textContent = "";
  status.textContent = "";
  select(0);
  query.focus();
}

async function paste() {
  if (pasting) {
    return;
  }
  pasting = true;
  status.textContent = "Pasting";
  try {
    await invoke("paste_sentinel");
    status.textContent = "";
  } catch (error) {
    status.textContent = `Paste failed: ${error}`;
  } finally {
    pasting = false;
  }
}

async function hide() {
  try {
    await invoke("panel_hide");
  } catch (error) {
    status.textContent = `Hide failed: ${error}`;
  }
}

listen("panel-shown", () => {
  reset();
  requestAnimationFrame(() =>
    requestAnimationFrame(() => invoke("panel_ready")),
  );
});

document.addEventListener("keydown", (event) => {
  keyLog.textContent = `Last key: ${event.key}`;
  if (event.key === "ArrowDown") {
    event.preventDefault();
    select(selected + 1);
  } else if (event.key === "ArrowUp") {
    event.preventDefault();
    select(selected - 1);
  } else if (event.key === "Enter") {
    event.preventDefault();
    paste();
  } else if (event.key === "Escape") {
    hide();
  }
});
