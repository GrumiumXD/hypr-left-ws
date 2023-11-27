# hypr-left-ws

This is a small tool for simulating [leftWM](https://github.com/leftwm/leftwm) workspace behaviour for a multi monitor setup in hyprland.

If the workspace you want to switch to, is currently assigned to a different monitor, it will first be moved to the current active monitor and then the switch happens. In case the target workspace is currently *visible* on a different monitor, both monitors will swap their workspaces.

So no matter which workspace you want to switch to, it will end up on the currently focused monitor.

## Installation

### Using Cargo
```bash
cargo install hypr-left-ws
```

## Usage
After installation the executable can be used as a drop-in replacement for the normal **workspace** dispatcher.

Just change your existing workspace switching *binds* inside your `hyprland.conf` to the following:
```
bind=$mod, 1, exec, hypr-left-ws 1
bind=$mod, 2, exec, hypr-left-ws 2
bind=$mod, 3, exec, hypr-left-ws 3
bind=$mod, 4, exec, hypr-left-ws 4
...
```