---
title: Building a terminal music player with Ratatui
sub_title: Rust Dortmund 2026
author: Orhun Parmaksız
theme:
  name: catppuccin-mocha
  override:
    footer:
      style: template
      left: "@orhundev"
      right: ""
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

![image:width:80%](assets/rat-cook.png)

<!-- column: 1 -->

<!-- new_lines: 2 -->

## **Orhun Parmaksız**

🐭 Lead maintainer of **Ratatui**

🧠 Rust Developer Advocate at JetBrains

📦 **Arch Linux** (btw)

---

`https://github.com/orhun`

`https://youtube.com/@orhundev`

<!-- end_slide -->

<!-- alignment: center -->

<!-- new_lines: 3 -->

![image:width:15%](assets/rat-chef.gif)

# We will build:

`A terminal MP3 player with Ratatui`

[](https://github.com/orhun/ratatui-music-player-workshop)

<!-- end_slide -->

# About the workshop

<!-- pause -->

<!-- column_layout: [3, 2] -->

<!-- column: 1 -->

![image:width:70%](assets/ratatui-spin.gif)

<!-- column: 0 -->

## Goals

- Get started with Rust & Ratatui
- Grasp the terminal UI concepts
- Build a local MP3 player for the terminal

<!-- pause -->

## Structure

- 2 hours in total
- Split up into 5 chapters
- Hands-on coding

<!-- pause -->

## Requirements

- Laptop with Rust installed
- Code editor with Rust support
- Terminal emulator

<!-- pause -->

<!-- end_slide -->

# What are we gonna cook today? 👨‍🍳

```bash +exec +acquire_terminal
cargo run
```

<!-- end_slide -->

# Ratatune 🎶

- Scan local MP3s from `music/`
- Browse a playlist
- Play, pause and skip tracks
- See track details and playback progress

<!-- pause -->

## Notes

- Already prepared:
  - File scanning
  - MP3 metadata
  - Audio output
- We will focus on:
  - Application state
  - Layout
  - Widgets
  - Input
- No network or streaming services

<!-- pause -->

`Use AI as less as possible to get the most out of the workshop!`

<!-- end_slide -->

# Schedule

| Duration   | Chapter       | Topic                                                              |
| ---------- | ------------- | ------------------------------------------------------------------ |
| **10 min** | **Chapter 1** | **Setup** - Generate a project from the Ratatune workshop template |
| **15 min** | **Chapter 2** | **Manage State** - Model a playlist and load local MP3 files       |
| **30 min** | **Chapter 3** | **Layout & Styling** - Arrange the playlist, player and controls   |
| **45 min** | **Chapter 4** | **Widgets** - Render a table, track details and progress gauge     |
| **20 min** | **Chapter 5** | **Interactivity** - Navigate, search and control playback          |
| **60 min** | 🧀            | **Q&A** / **Debugging** / **Bonus tracks**                         |

<!-- end_slide -->

# Chapter 1 - Setup 🧀 (10 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Install Rust using `rustup.rs` or any other way

• Install `cargo-generate`

• Generate a project from this workshop repository

• Select `template` when prompted, then enter `ratatune` as the project name

```bash
cargo install --locked cargo-generate

cargo generate --git https://github.com/orhun/ratatui-music-player-workshop
```

• The starter already includes `music.rs` and the audio dependencies

<!-- column: 1 -->

## Bonus track

• Change the title and colors

• Add a `music/` folder with a few MP3 files

• Run `cargo build` once and check audio output before the workshop

<!-- end_slide -->

# Chapter 2 - Manage state 🧀 (15 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Use the prepared `music.rs` module to scan local MP3s

• Store the list of `Track`s in `App`

• Show the title, artist and track length

• Show a helpful empty-playlist message

<!-- column: 1 -->

## Bonus track

• Show how many tracks were found

• Add an error message when `music/` cannot be read

<!-- end_slide -->

# Chapter 3 - Layout & styling 🧀 (30 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Split the screen into a header, playlist, player and footer

• Show the playlist beside the now-playing panel

• Add a progress area below the track details

<!-- column: 1 -->

## Bonus track

• Add a one-line title at the top

• Refactor each area into its own render function

• Render rounded borders and a styled header

<!-- end_slide -->

# Chapter 4 - Widgets 🧀 (45 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Render tracks in a `Table`

• Highlight the selected track

• Show title, artist, album and duration

• Render playback progress with a `Gauge`

<!-- column: 1 -->

## Bonus track

• Show a playing icon beside the current track

• Adjust volume with `+`/`-`

• Add a visualizer with `Sparkline`

<!-- end_slide -->

# Chapter 5 - Interactivity 🧀 (20 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Move through the playlist with `j`/`k` or arrow keys

• Play the selected track with `Enter`

• Pause and resume with `Space`

• Search the playlist with `/`

<!-- column: 1 -->

## Bonus track

• Play the next track with `n`

• Add previous, volume, repeat, or shuffle controls

• Seek within the current track

<!-- end_slide -->

# Extras (60 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

• Q&A

• Show & tell!

• Start building your own Ratatui app!

<!-- column: 1 -->

![](assets/rat-cheese.gif)
