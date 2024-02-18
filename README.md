# DISTRO

This monorepo holds the projects that build into the necessary binaries to get my personal distro working.

The different projects are designed so they can be independently recicled for other setups. For example: you can make use of the workspaces daemon that hyppprctl provides to make your waybar functionality, or you can set up a daemon for sway that reproduces hyppprctl's and change window manager.

[ ] This explanation sucks a little asik TODO() para cuando el proyecto esté más avanzado.

## Hyppprctl\*

[dependencies="hypr"]
Hypprctl has a double functionality. Firstly, it centralizes all tools held in this repo into the same command line interface.

Also, it is a wrapper for hyprctl (made with hyprland-rs) that provides extended utilities for hyprland.

```
hypprctl <arguments>
```

## Ewwctl

```
ewwctl <arguments>
hypprctl eww <arguments>
```

[dependencies="eww"]
This binary contains the daemons responsible for eww variables and windows state management.

## Audioctl

```
audioctl <arguments>
hypprctl audio <arguments>
```

[dependencies="pipewire"]
This binary contains the daemons that print the state of audio devices and provides commands to change that state.
