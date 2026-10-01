# r0-common

Timing harness shared by the R0 spikes (criterion R0-6, NFR-1 and NFR-4). It has no dependencies and no Tauri code, so every spike measures the same way on every OS.

The crate is standalone: it has its own empty `[workspace]` table and is not a member of the root Cargo or pnpm workspace. A spike depends on it by path and carries its own empty `[workspace]` table too:

```toml
[dependencies]
r0-common = { path = "../r0-common" }

[workspace]
```

## Shortcut to panel ready

Pass: p95 at most 150 ms over 50 opens, from the Rust shortcut handler to the webview's first painted frame.

```rust
use r0_common::OpenTimer;

let timer = OpenTimer::new();
timer.shortcut_fired();
timer.first_frame_acked();
println!("{}", timer.report());
```

- Keep one `OpenTimer` in app state, and use a fresh one for each run of 50. Call `shortcut_fired` as the first statement of the global-shortcut handler, before the panel is shown, and only for the key press that opens the panel, not for the key release or a press that hides it. A second `shortcut_fired` before the ack replaces the first.
- The webview calls an IPC command once per open, after two `requestAnimationFrame` callbacks, so the ack follows a painted frame. The command calls `first_frame_acked`:

```ts
requestAnimationFrame(() => requestAnimationFrame(() => invoke("first_frame_ack")));
```

- Start the `requestAnimationFrame` pair only after the panel is visible, for example from an event Rust emits right after it shows the panel. A hidden webview pauses `requestAnimationFrame`, and the two frames and the event hop count toward the measured time.
- Measure warm: create and load the panel's webview at launch, hidden, before the first open.
- One open at a time: show the panel, wait for the ack, hide the panel, repeat until the report says 50 opens.
- `first_frame_acked` returns `None` when no shortcut is pending. A `shortcut_fired` that never gets an ack is dropped, so the report shows fewer than 50 opens instead of a skewed p95.
- Paste the final `println!` line into the results report. It reads like `shortcut_to_panel_ready opens=50 p95=48ms max=50ms budget=150ms PASS`.

## Enter to text visible

Pass: at most 300 ms in each of 5 runs. This is a screen-recording spot check, not a harness measurement. At 60 fps, 300 ms is 18 frames. A frame count is off by up to one frame in each direction, so the limit below is 17 frames.

1. Open the target app with the caret in an empty text field. Turn on a keystroke overlay so the screen shows when ↵ is pressed.
2. Record the screen at 60 fps. Check the frame rate of the saved file.
3. Open the panel with the shortcut, select the sentinel item, press ↵.
4. Step through the recording frame by frame. Frame A is the first frame that shows ↵ in the overlay. Frame B is the first frame that shows the sentinel text in the target.
5. Record B minus A in frames. Repeat for 5 runs. Every run must be 17 frames or fewer. The overlay draws after the key event, so the count does not include the overlay's own lag.
6. Note the recorder, the overlay tool and the frame counts of all 5 runs in the results report.
