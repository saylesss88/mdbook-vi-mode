# mdbook-vi-mode

An [mdBook](https://rust-lang.github.io/mdBook/) preprocessor that adds
Vim-style keyboard navigation to the rendered HTML book. A single cursor moves
through the page and jumps between the chapter list in the sidebar and the text
in the body.

## Installation

```bash
cargo install --path .
```

Then enable it in your book's `book.toml`:

```toml
[preprocessor.vi-mode]
```

Run `mdbook build` (or `mdbook serve`) as usual.

## Usage

Navigation starts in **reading** mode, where every keystroke passes straight
through to the browser and mdBook. Press the toggle key (`` ` `` by default) to
enter **nav** mode; a small `VI` badge appears in the corner and the cursor
becomes visible. Press it again — or `Escape` — to return to reading.

| Key            | Action                                              |
| -------------- | --------------------------------------------------- |
| `` ` ``        | Toggle navigation on / off                          |
| `Escape`       | Return to reading mode                              |
| `j` / `↓`      | Move cursor down                                    |
| `k` / `↑`      | Move cursor up                                       |
| `h`            | Jump to the sidebar (chapter list)                  |
| `l`            | Jump back to the page content                       |
| `g` `g`        | Jump to the first item                              |
| `G`            | Jump to the last item                               |
| `Enter` / `o`  | Follow the chapter link, or a link under the cursor |

Keys are ignored while a text input is focused (such as mdBook's search box), so
they never interfere with typing.

## Configuration

All options are optional and live under `[preprocessor.vi-mode]`:

```toml
[preprocessor.vi-mode]
toggle-key   = "`"        # KeyboardEvent.key that toggles navigation
start-active = false      # show the cursor immediately on page load
cursor-color = "#e46876"  # any CSS color
```

| Option         | Default     | Description                                                      |
| -------------- | ----------- | ---------------------------------------------------------------- |
| `toggle-key`   | `` "`" ``   | The [`KeyboardEvent.key`] value that turns navigation on and off |
| `start-active` | `false`     | Whether the cursor is shown before the toggle key is pressed     |
| `cursor-color` | `"#e46876"` | Color of the cursor outline and the sidebar highlight            |

[`KeyboardEvent.key`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key

Invalid configuration is reported as a warning and ignored; it never fails the
build.

## How it works

An mdBook preprocessor runs *before* rendering and only transforms the book's
Markdown content, it has no access to the final page or its cursor. So the
navigation itself is plain client-side JavaScript, and the preprocessor's only
job is to inject that script (and its stylesheet) into every chapter.

Because mdBook renders each chapter as a standalone HTML page that always
includes the sidebar, the injected script can move one cursor between the
chapter links (`#mdbook-sidebar a`) and the page body (`#mdbook-content main`).
The sidebar is populated asynchronously by mdBook's `toc.js`, so the script
queries it live and repaints via a `MutationObserver` once it is ready. The
active state and current zone are stored in `sessionStorage`, so following a
chapter link keeps you where you were.

Configuration is serialized into a small `window.__viModeConfig` object that the
bundled script reads at runtime, which keeps `vi-mode.js` a self-contained file
with its own defaults.

## Renderer support

Only the `html` renderer is supported; the preprocessor is a no-op for any
other renderer.

## License

