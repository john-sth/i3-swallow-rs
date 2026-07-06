//======================================================
// i3-swallow-rs
//
// A Rust re-implementation of i3-swallow[1], a small utility for the i3
// window manager that makes terminal windows "swallow" the GUI windows
// they launch.
//
// Planned flow (mirrors the original Python implementation):
//   1. Connect to the running i3 instance via IPC and record the currently
//      focused window (the terminal).
//   2. Spawn the requested command as a child process.
//   3. Listen for "window::new" events; when one fires, move the focused
//      (terminal) window to the scratchpad and mark it as swallowed.
//   4. Listen for "window::close" events; once the spawned process exits,
//      restore the terminal from the scratchpad.
//
// [1] https://github.com/jamesofarrell/i3-swallow
//======================================================
use i3ipc::I3Connection;

fn main() {
    let mut connection = I3Connection::connect().expect("failed to connect to i3");
    let version = connection.get_version().expect("failed to get version");
    println!("connected to i3, version: {}", version.human_readable);

    let tree = connection.get_tree().expect("failed to get tree");
    println!("root node id: {}, type: {:?}", tree.id, tree.nodetype);
}
