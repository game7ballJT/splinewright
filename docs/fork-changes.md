# What this fork changes

Splinewright is a fork of [VectorCraft](https://github.com/storytold/vectorcraft) (upstream commit
`4f07bd9`, 2026-10-08). The upstream history is preserved in this repository, so `git log` shows who
wrote what; this page lists what the fork changed on top of it.

## Rebrand

- Every crate, binary, app ID, string, document and test is renamed: `vectorcraft-*` is
  `splinewright-*`, the app ID is `online.jonmcgee.splinewright`, the native format extension is
  `.splinewright`.
- The upstream brand is removed: `docs/brand/` (its logos are not open source), the Discord and
  website links, the family table. The Help menu links to this repository and, as the credit the
  fork owes, to the upstream project. A new, original app icon (a Bezier curve with its anchors and
  handles) is drawn in code (`packaging/icons.ps1`).
- Legal files kept and corrected: `LICENSE-MIT` names the original copyright holders,
  `LICENSE-APACHE` is untouched, `NOTICE` and `ASSETS.md` say what is upstream and what is new.

## New features

| Feature | Command | Notes |
|---|---|---|
| Check Spelling | `type.checkSpelling`, `type.spell.change`, `type.spell.addWord`, `type.spell.suggest` | Edit › Spelling › Check Spelling… (Cmd+I) opens a dialog (Change, Change All, Ignore, Ignore All, Add); bundled SCOWL English dictionary (`assets/dict`), a custom dictionary in Preferences › Type (Edit › Spelling › Edit Custom Dictionary…). The speller is the PhotoCraft/Pixelwright one, ported |
| Check Contrast | `object.checkContrast` | WCAG 2 ratio between two objects (or one and a background colour) and the levels it meets; Object › Check Contrast shows it in the status bar |
| Help (F1) | `help.docs` | Opens the documentation folder of this repository: finishes the stubbed Help item |
| Project links | `help.website`, `help.github`, `help.upstream`, `help.reportIssue` | Replace the upstream community links |
| `splinewright-cli palette` | CLI | The colours a document is drawn with, most used first (text, hex or JSON) |
| `splinewright-cli contrast` | CLI | Contrast ratio of two colours; exit code 1 below AA |
| Web `?open=` and examples | `splinewright-web` | `?open=samples/<name>.splinewright` opens a bundled example (`examples/` is copied next to the page); same-origin paths only (`is_same_origin_path`) |
| `wasm-release` profile | `Cargo.toml`, `apps/splinewright-web/index.html` | Size-optimised browser build (opt-level `s`, fat LTO, full speed for the geometry, render and path-boolean crates); the plain release profile gave a 43 MiB `.wasm`, over the 24 MiB gate |

Everything above is a registered command (so it is in the menus, the command search, the control
channel and MCP), has unit tests, and has translations in the interface languages that cover menus.