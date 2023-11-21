- [scrim](#scrim)
  - [Installing](#installing)
  - [Controls](#controls)
  - [Navigation](#navigation)
  - [Autocomplete and Lookups](#autocomplete-and-lookups)
    - [Adding Lookups](#adding-lookups)
  - [Notes](#notes)

# scrim 
`scrim`, The **S**uper **C**ool **R**ust Char**I**cter **M**anager (yes, we do know how to spell character correctly, but it didn't fit with the acronym), simple dnd character manager written in rust, with vim-inspired tui controls for easy (if you know vim) editing. It offers a simple autocomplete and lookup system for maximum convenience.

## Installing
The `scrim` binary can be installed locally to your machine by running either `install.sh` or `install.bat`, depending on your platform. The install script with place a release build of the application in the users path and copy necessary files. Once installed, the command
```
scrim [player]
```
can be used to launch a `scrim` session. If a player name is given, then `scrim` will attempt to load the specified player file.

## Controls
`scrim` is (kinda) a modal editor, which means that you will have to used keybinds to switch between differents modes. The following keybinds can be used to change the selected control:  

- `u` - highlight the top bar.
- `i` - highlight the info bar.
- `s` - highlight the stat pane.
- `t` - highlight the current tab pane (Notes, Inventory, or Spells).
- `1` - select the notes pane.
- `2` - select the inventory pane.
- `3` - select the spells pane.
- `E` - open the spell slots menu.
- `l` - (with a line in the tab menu selected) - open a lookup for the current line.
- `L` - (tab pane not selected) - open the free lookup box.
- `C` - attempt to open a lookup for the current class.
- `R` - attempt to open a lookup for the current race.
- `F` - open the funds menu.
- `P` - open the proficiencies menu.
- `[` - open the player select menu.
- `S` - save the player.

Fortunately, most selectable controls will have the shortcut that selects them placed in parentheses somwhere near their name, so if you forget just try to look as best as you can.  

## Navigation
- `h`, `l` - navigate left and right in the current widget.
- `j`, `k` - navigate up and dowwn in the current pane. If no pane is selected, this will scroll the current tab pane. If editing mode is engaged, this will scroll through numerical values and other values such as the player class, race, background, and alignment.
- `tab` - while typing, access the autocomplete menu. If used in the free lookup box functions identically to pressing enter.
- `enter` - enter editing mode or select an item from a list.
- `a`, `x` - increase and decrease values in certain panes, namely money and spell slots remaining.
- `A`, `X` - increase and decrease money by 10, increase and decrease total spell slots.
- `esc` - if editing mode is engaged, exit editing mode. if a pane is selected, deselect the pane.
- `q` - same functions as esc, but if no pane is selected, open the quit confirmation menu.

## Autocomplete and Lookups
`scrim` has an easy-to-use autocomplete and lookup system. `C`, `R`, `l`, and `L` can be used to access lookups, and `tab` can be used to perform autocomplete. The lookup entries are loaded dynamically at startup. If the application is run in debug mode, then the `lookups` folder in the base folder of the repository is assumed to contain lookups. Otherwise, the `.scrim` folder in the user's home directory is assumed to contain lookups. Currently, lookups for spells, weapons, races, classes, and subclasses are provided by default, and will be placed in the correct folders when the install script is run.

### Adding Lookups
Lookups are stored in the `JSON` file format and should be placed in the appropriate `lookups` folder prior to startup. Only `JSON` files will be attempted to be loaded by `scrim`. Each lookup should have a top level dictionary containing exactly one entry -- a dictionary called `entries`. Each entries to entries should have a key in all lowercase that represents the name of the lookup, and the entry itself should be another dictionary. Each entry should contain three entries:  

- `name`: the name of the item with any proper formatting necessary.
- `description_short`: an optional, short description of the entry.
- `description`: the full body of the entry.

Due to the limitations of the `JSON` format, every field must be containted on one line, so manually escaped `\n` newlines will have to be used in place. This is unfortunate but will be necessary unless a different file format is chosen, which is highly unlikely.

## Notes
- If a pane is selected, the currently selected pane item will be highlighted in yellow. If an item is selected and is currently being edited, it will be highlighted in green.
- If no player name is passed to `scrim` on startup, then the player name specified in the app will be used to save the file.
- If you want to save a player to a different file, then open a new terminal window, move the old file, save the player, move the new file, and then move the old file back. A `save as` feature may be implemented but it is currently not very high priority
