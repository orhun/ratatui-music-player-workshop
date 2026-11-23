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

![image:width:80%](assets/ratatui-spin.gif)

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

`Use AI as little as possible to get the most out of the workshop!`

<!-- end_slide -->

# Schedule

| Duration   | Chapter       | Topic                                                              |
| ---------- | ------------- | ------------------------------------------------------------------ |
| **10 min** | **Chapter 1** | **Setup** - Create the project from template                       |
| **15 min** | **Chapter 2** | **Manage State** - Load local MP3 files and show them              |
| **30 min** | **Chapter 3** | **Widgets** - Render a table and navigate with the arrow keys      |
| **45 min** | **Chapter 4** | **Layout & Styling** - Split the screen into multiple widget areas |
| **20 min** | **Chapter 5** | **Interactivity** - Navigate, search and control playback          |
| **60 min** | 🧀            | **Q&A** + **Bonus tracks**                                         |

<!-- end_slide -->

# Chapter 1 - Setup 🧀 (10 min)

## Objectives

• Install Rust (e.g. via `rustup.rs`)

• Install `cargo-generate`

```bash
cargo install --locked cargo-generate
```

• Generate a project from this workshop repository

```bash
cargo generate --git https://github.com/orhun/ratatui-music-player-workshop
```

• Look around and run `cargo run` once and check audio output.

## Bonus track

• 1) Change the title and colors 2) Add your own mp3 files

<!-- end_slide -->

# Chapter 2 - Manage state 🧀 (15 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Store the `Vec<music::Track>` in `App`

• Load tracks with `music::scan_tracks`

• Show the tracks in a `Paragraph` widget

<!-- column: 1 -->

## Bonus track

• Format track length with `music::format_duration`

• Show how many tracks were found

• Add an error message when `music/` cannot be read

<!-- end_slide -->

# Chapter 3 - Widgets 🧀 (30 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Render the playlist in a `Table`

• Show title, artist and track length

• Highlight the selected track

• Move through tracks with the up and down arrow keys

<!-- column: 1 -->

## Bonus track

• Also support `j`/`k` navigation

• Style alternating rows for easier reading

• Use a symbol such as `▶` to mark the selected row

<!-- end_slide -->

# Chapter 4 - Layout & styling 🧀 (45 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Use `Layout` to split the screen into a header, playlist and footer

• Show playback progress with a `Gauge`

• Render a separate widget in each area

<!-- column: 1 -->

## Bonus track

• Refactor each area into its own render function

• Render rounded borders and a styled header

• Add a selected-track details panel

<!-- end_slide -->

# Chapter 5 - Interactivity 🧀 (20 min)

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Objectives

• Play the selected track with `Enter`

• Pause and resume with `Space`

• Play the next track with `n`

• Add previous, volume, repeat or shuffle controls

<!-- column: 1 -->

## Bonus track

• Show a popup with the keyboard shortcuts when pressing `?`

• Search the playlist with `/`  
(use `ratatui-textarea`)

• Add equalizer (use `tui-equalizer`)

<!-- end_slide -->

# Extras (60 min)

• Work on bonus chapters!

• Q&A!

• Show & tell! (`https://github.com/ratatui/awesome-ratatui#music-and-audio`)

• Start building your own Ratatui app!

![image:width:30%](assets/rat-cheese.gif)
