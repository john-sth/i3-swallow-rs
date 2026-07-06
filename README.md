# i3-swallow-rs

Rust re-implementation of [i3-swallow](https://github.com/jamesofarrell/i3-swallow), a small
utility for the [i3](https://i3wm.org/) window manager that makes a terminal window "swallow"
the GUI windows it launches.

## What it does

Run a GUI program from your terminal (an image viewer, a PDF reader, a file manager, ...) and,
instead of the new window popping up next to your terminal, the terminal is tucked away into the
scratchpad and the GUI window takes its place. When you close the GUI window, the terminal
reappears right where it was. Your workspace stays uncluttered when launching short-lived GUI
apps from a terminal-driven i3 workflow.

## How it works

1. Connect to the running i3 instance over its IPC socket and record the currently focused
   window (the terminal).
2. Spawn the requested command as a child process.
3. Listen for `window::new` events. When one arrives, move the focused (terminal) window to the
   scratchpad and mark it as swallowed.
4. Listen for `window::close` events. Once the spawned process has exited, restore the terminal
   from the scratchpad.

## Usage (planned)

```sh
i3-swallow-rs [-d] <cmd> [args...]
```

- `<cmd>`: the command to run (e.g. `feh image.png`)
- `-d`: don't return the terminal window once the command exits

Typical i3 config binding:

```
bindsym $mod+Return exec i3-swallow-rs alacritty
```

## Project status

This is an early-stage rewrite of the original Python tool. Progress so far:

- [x] i3 IPC connection established and verified (`get_version`, `get_tree`)
- [ ] CLI argument parsing (`clap`)
- [ ] Spawn child process (`subprocess`)
- [ ] Track focused window and swallow it on `window::new`
- [ ] Restore window on `window::close` / process exit
- [ ] `-d` flag behavior (don't restore)

## Rewrite notes

Candidate crates for each piece of the original Python implementation:

| Python                         | Rust crate      |
|---------------------------------|-----------------|
| `argparse`                       | `clap`          |
| `i3ipc`                          | `i3ipc`         |
| `subprocess`                     | `subprocess`    |
| `sys`                             | `subprocess`    |
| `threading.Timer`                | `thread_timer`  |
| `time.sleep`                      | `std::thread::sleep` |

## Building

```sh
cargo build --release
```

Requires a running i3 session to connect to at runtime.
