const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const search = document.getElementById("search");
const status = document.getElementById("status");
const permission = document.getElementById("permission");
const rows = [...document.querySelectorAll("#items li")];
let selected = 0;

function select(index) {
  selected = (index + rows.length) % rows.length;
  rows.forEach((row, rowIndex) => row.setAttribute("aria-selected", String(rowIndex === selected)));
}

function paste() {
  status.textContent = "Pasting…";
  invoke("panel_paste").catch((error) => {
    status.textContent = String(error);
  });
}

listen("panel:show", (event) => {
  permission.hidden = event.payload !== "permission";
  status.textContent = "";
  search.value = "";
  select(0);
  search.focus();
  requestAnimationFrame(() => requestAnimationFrame(() => invoke("panel_ready", { permissionShown: !permission.hidden })));
});

document.addEventListener("keydown", (event) => {
  invoke("panel_key", { key: event.key });
  if (event.key === "Escape") {
    invoke("panel_hide");
  } else if (event.key === "Enter") {
    paste();
  } else if (event.key === "ArrowDown") {
    event.preventDefault();
    select(selected + 1);
  } else if (event.key === "ArrowUp") {
    event.preventDefault();
    select(selected - 1);
  }
});

document.getElementById("open-settings").addEventListener("click", () => {
  invoke("open_accessibility_settings");
});
