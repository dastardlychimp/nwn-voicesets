# Features

- create/edit/delete voiceset for players
- Export Multiple soundsets

## Export Multiple Soundsets (Voiceset)
- Copy `dialog.tlk`
- Copy `soundset.2da`
- For each voiceset
  - Modify tlk table
  - Create a SSF
  - Add ssf reference to `soundset.2da`
  - Bundle up .ssf, .wav files to a single .hak file.
- Bundle up `dialog.tlk`, `soundset.2da`, and `voiceset.hak`s.

## Player Voiceset
- Store voiceset data: `.wav` files and corresponding text.
- Edit voiceset data.
- Delete voiceset data.

# Possible Future Features
- create voiceset for **Henchman**, **NPC Full**, **NPC part**, and **Monster**.
- Edit playlists can open/sample sounds.

