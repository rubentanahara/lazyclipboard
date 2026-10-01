const { invoke } = window.__TAURI__.core;
const key = document.getElementById("key");
const status = document.getElementById("status");

async function pasteSentinel() {
  status.textContent = "Writing the sentinel to the clipboard...";
  try {
    const text = await invoke("paste_sentinel");
    status.textContent = "Placed on the clipboard: " + text;
  } catch (error) {
    status.textContent = "Clipboard write failed: " + error;
  }
}

document.addEventListener("keydown", (event) => {
  key.textContent = event.key;
  if (event.key === "Enter" && !event.repeat) pasteSentinel();
  if (event.key === "Escape") invoke("hide_panel");
});
