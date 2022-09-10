# Sway Workspace Manager
A dynamic workspace manager for sway.

This project is an interface for sway workspace operations.
It ensures that after any operation, workspace numbers will correspond to their indices on the list of open workspaces.
Most notably, this program allows you to **create a workspace between any two existing workspaces**.

This program only manages workspace numbers. "Workspace names" that follow the numbers will be left untouched.

## Usage
```
sway-workspace-manager VERB TARGET [--daemon] [--cycle] [--extra]
```

Supported verbs are `reorder` `switch` `move` `create` `move-to-new` `swap` `rename`.

`reorder` will only fix the workspace numbers, and does not require a target.
`rename` renames the current workspace to `workspace_num:TARGET`.

For all other verbs, TARGET must be `prev` `next` `start` `end` or a 1-indexed workspace number.

`--daemon` only applies to the `reorder` verb, and sets the program to reorder automatically on "workspace close" events.

`--cycle` only affects the program's behavior when the target is `prev` or `next`, and when the verb is `switch` `move` or `swap`.

When the target is a number, `--extra` will allow `switch` and `move` to create a new workspace at the end of the workspace list.

## Examples

### Config

`~/.config/sway/config`
```ini
# switch workspace
bindsym Mod4+Left  exec sway-workspace-manager switch prev --cycle
bindsym Mod4+Right exec sway-workspace-manager switch next --cycle

# create workspace
bindsym Mod4+Ctrl+Left  exec sway-workspace-manager create prev
bindsym Mod4+Ctrl+Right exec sway-workspace-manager create next

# move focused window between workspaces
bindsym Mod4+Shift+Left  exec sway-workspace-manager move prev --cycle
bindsym Mod4+Shift+Right exec sway-workspace-manager move next --cycle

# move focused window to new workspace
bindsym Mod4+Ctrl+Shift+Left  exec sway-workspace-manager move-to-new prev
bindsym Mod4+Ctrl+Shift+Right exec sway-workspace-manager move-to-new next

# swap workspace order
bindsym Mod4+Alt+Left  exec sway-workspace-manager swap prev --cycle
bindsym Mod4+Alt+Right exec sway-workspace-manager swap next --cycle


# switch to a workspace number
bindsym Mod4+1 exec sway-workspace-manager switch 1
bindsym Mod4+2 exec sway-workspace-manager switch 2
...
bindsym Mod4+9 exec sway-workspace-manager switch 9
bindsym Mod4+0 exec sway-workspace-manager switch end


# create workspace with a number
bindsym Mod4+Ctrl+1 exec sway-workspace-manager create 1

# move focused window to a workspace number
bindsym Mod4+Shift+1 exec sway-workspace-manager move 1


# rename current workspace (using dmenu-wl)
bindsym Mod4+slash exec 'out=$(dmenu-wl -po "") && sway-workspace-manager rename "$out"'
```

### Reorder Daemon

`~/.config/systemd/user/sway-workspace-manager.service`
```service
[Unit]
Description=A dynamic workspace manager for sway
PartOf=sway-session.target
After=sway-session.target

[Service]
ExecStart=sway-workspace-manager reorder --daemon

[Install]
WantedBy=sway-session.target
```

Change `sway-session.target` to `graphical-session.target` if you do not use systemd to manage sway.

Or just start it from sway:

`~/.config/sway/config`
```ini
exec sway-workspace-manager reorder --daemon
```
