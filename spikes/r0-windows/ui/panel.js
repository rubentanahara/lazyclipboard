const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const SENTINEL = "LAZYCLIPBOARD-R0-SENTINEL";
const rows = [SENTINEL, "LAZYCLIPBOARD-R0-ROW-2", "LAZYCLIPBOARD-R0-ROW-3"];

const query = document.getElementById("query");
const list = document.getElementById("rows");
const keys = document.getElementById("keys");
const error = document.getElementById("error");

let selected = 0;
let keyCount = 0;

function render() {
  list.replaceChildren(
    ...rows.map((text, index) => {
      const row = document.createElement("li");
      row.setAttribute("role", "option");
      row.setAttribute("aria-selected", String(index === selected));
      row.textContent = text;
      return row;
    }),
  );
}

function reset() {
  selected = 0;
  keyCount = 0;
  query.textContent = "";
  keys.textContent = "No key received yet";
  error.textContent = "";
  render();
}

function showError(message) {
  error.textContent = String(message);
}

function move(step) {
  selected = (selected + step + rows.length) % rows.length;
  render();
}

function onKey(event) {
  keyCount += 1;
  keys.textContent = `Key #${keyCount}: ${event.key}${event.shiftKey ? " (Shift held)" : ""}`;
  if (event.key === "ArrowDown") move(1);
  else if (event.key === "ArrowUp") move(-1);
  else if (event.key === "Enter" && !event.repeat) {
    invoke("paste", { text: rows[selected] }).catch(showError);
  } else if (event.key === "Escape") invoke("panel_hide").catch(showError);
  else if (event.key.length === 1) query.textContent += event.key;
}

document.addEventListener("keydown", onKey);

listen("panel-shown", () => {
  reset();
  requestAnimationFrame(() => requestAnimationFrame(() => invoke("panel_ready")));
});

render();
