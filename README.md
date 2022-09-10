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
