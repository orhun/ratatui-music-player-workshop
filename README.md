![](assets/workshop-banner.png)

In this hands-on workshop, you will use [Ratatui](https://ratatui.rs), the Rust framework for building terminal user interfaces, to build **Ratatune**: a terminal music player for local MP3 files.

### What will you build?

**Ratatune** scans the `music/` folder, displays the MP3 files it finds, and lets you browse, search, play and pause your music. The finished app shows the current track and playback progress!

### Who is this workshop for?

The workshop is intended for Rust beginners who want to learn how to build TUIs with Ratatui. Previous experience with Rust is not required, but a basic understanding of the language is recommended.

- If you are already familiar with Rust, you can still follow along and learn about Ratatui.
- If you are already familiar with Ratatui, follow the _bonus tracks_ and add more features!

#### Necessary Tools

- [Rust](https://www.rust-lang.org/tools/install) and `cargo`
- [git](https://git-scm.com/)
- A code editor with Rust support
- A terminal emulator
- A few local MP3 files in the `music/` folder
- Headphones or speakers for playback

> [!NOTE]
> On Linux, install the ALSA and PulseAudio development packages before building the audio dependencies (for example, `libasound2-dev` and `libpulse-dev` on Debian/Ubuntu).

> [!TIP]
> It is recommended to disable AI completions in your code editor to get a better understanding of the code you write.

### Chapters

The workshop is split into 5 chapters and aims to be completed **in 3 hours**.

| Duration   | Chapter       | Topic                                                             |
| ---------- | ------------- | ----------------------------------------------------------------- |
| **10 min** | **Chapter 1** | **Setup** - Generate a project from the Ratatune workshop template |
| **15 min** | **Chapter 2** | **Manage State** - Model a playlist and load local MP3 files      |
| **30 min** | **Chapter 3** | **Widgets** - Render a playlist table and navigate with the arrow keys |
| **45 min** | **Chapter 4** | **Layout & Styling** - Split the screen into multiple widget areas   |
| **20 min** | **Chapter 5** | **Interactivity** - Navigate, search and control playback          |
| **60 min** | 🧀            | **Q&A** / **Debugging** / **Bonus tracks**                        |

### Walkthrough

Use the [slides](presentation.md) to follow along with the workshop.

> [!TIP]
> You can use [`presenterm`](https://github.com/mfontanini/presenterm) to present the slides in your terminal.
>
> ```sh
> presenterm -c assets/presenterm.yml presentation.md
> ```

Generate the workshop starter directly from this repository. The template configuration exposes `template/` as the only subtemplate, so you do not need to pass its path. Select `template` when prompted, then enter `ratatune` as the project name:

```sh
cargo install --locked cargo-generate
cargo generate --git https://github.com/orhun/ratatui-music-player-workshop
cd ratatune
cargo run
```
The generated project starts at Chapter 1, with `music.rs` already wired in. Put your MP3 files in its `music/` folder. Before the workshop, run `cargo build` and confirm your laptop can play audio.

If you get stuck, check the [chapter examples](./examples), which contain the completed code for each step. Try the exercises first and use the examples when you need a reference. The final app is in [src/main.rs](./src/main.rs), with the prepared music support in [template/src/music.rs](./template/src/music.rs).

## Show and tell! 🧀

Join the [Ratatui Discord](https://discord.gg/pMCEU9hNEj) and share your creations with the other terminal chefs!

If you have any questions or need help, feel free to ask in the chat.

## License

Copyright © 2025, [Orhun Parmaksız](https://github.com/orhun)

Licensed under [The MIT License](./LICENSE)

🦀 ノ( º \_ º ノ) - respect crables!
