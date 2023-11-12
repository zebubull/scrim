# scrim 
`scrim`, The **S**uper **C**ool **R**ust **I**tem **M**anager, simple dnd character manager written in rust, with vim-inspired tui controls for easy (if you know vim) editing.

## Use
The `scrim` binary can be installed locally to your machine by running `cargo install --path .`. With `scrim` installed, you can run  
```
scrim ...
```
to launch a `scrim` session. If a player name is passed, then `scrim` will attempt to load the specified player file. Spaces do not need to be escaped; `scrim` will automatically add spaces to the player name where necessary.

## Controls
- `b` - highlight the top bar.
- `i` - highlight the info bar.
- `s` - highlight the stat pane.
- `t` - highlight the current tab pane (Notes, Inventory, or Spells).
- `1` - select the notes pane.
- `2` - select the inventory pane.
- `3` - select the spells pane.
- `h`, `l` - navigate left and right in the current pane
- `j`, `k` - navigate up and dowwn in the current pane. If no pane is selected, this will scroll the current tab pane. If editing mode is engaged, this will scroll through numerical values and other values such as the player class, race, background, and alignment.
- `enter` - enter editing mode
- `esc` - if editing mode is engaged, exit editing mode. if a pane is selected, deselect the pane.
- `q` - quit the application

## Notes
- Only `j` and `k` have special functionality in editing mode. All other inputs will type their respective letters or do nothing (in the case of scroll inputs).
- If a pane is selected, the currently selected pane item will be highlighted in yellow. If an item is selected and is currently being edited, it will be highlighted in green.
- If no player name is passed to `scrim` on startup, then the player name specified in the app will be used to save the file.
